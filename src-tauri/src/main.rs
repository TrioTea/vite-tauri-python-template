// 在发布模式下防止额外的控制台窗口出现，请勿删除！！
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager, RunEvent};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;

#[tauri::command]
fn toggle_fullscreen(window: tauri::Window) {
    if let Ok(is_fullscreen) = window.is_fullscreen() {
        window.set_fullscreen(!is_fullscreen).unwrap();
    }
}

// 辅助函数，用于启动 sidecar 并监控其 stdout/stderr
fn spawn_and_monitor_sidecar(app_handle: tauri::AppHandle) -> Result<(), String> {
    // 检查是否已经存在 sidecar 进程
    if let Some(state) = app_handle.try_state::<Arc<Mutex<Option<CommandChild>>>>() {
        let child_process = state.lock().unwrap();
        if child_process.is_some() {
            // sidecar 已经在运行，不要启动新的进程
            println!("[tauri] Sidecar 已经在运行。跳过启动。");
            return Ok(()); // 提前退出，因为 sidecar 已经在运行
        }
    }
    // 启动 sidecar
    let sidecar_command = app_handle
        .shell()
        .sidecar("main")
        .map_err(|e| e.to_string())?;
    let (mut rx, child) = sidecar_command.spawn().map_err(|e| e.to_string())?;
    // 将子进程存储在应用状态中
    if let Some(state) = app_handle.try_state::<Arc<Mutex<Option<CommandChild>>>>() {
        *state.lock().unwrap() = Some(child);
    } else {
        return Err("无法访问应用状态".to_string());
    }

    // 启动异步任务处理 sidecar 通信
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    println!("Sidecar stdout: {}", line);
                    // 向前端发送输出
                    app_handle
                        .emit("sidecar-stdout", line.to_string())
                        .expect("发送 sidecar stdout 事件失败");
                }
                CommandEvent::Stderr(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    eprintln!("Sidecar stderr: {}", line);
                    // 向前端发送错误输出
                    app_handle
                        .emit("sidecar-stderr", line.to_string())
                        .expect("发送 sidecar stderr 事件失败");
                }
                _ => {}
            }
        }
    });

    Ok(())
}

// 定义关闭 sidecar 进程的命令
#[tauri::command]
fn shutdown_sidecar(app_handle: tauri::AppHandle) -> Result<String, String> {
    println!("[tauri] 收到关闭 sidecar 的命令。");
    // 访问 sidecar 进程状态
    if let Some(state) = app_handle.try_state::<Arc<Mutex<Option<CommandChild>>>>() {
        let mut child_process = state
            .lock()
            .map_err(|_| "[tauri] 获取 sidecar 进程锁失败。")?;

        if let Some(mut process) = child_process.take() {
            let command = "sidecar shutdown\n"; // 添加换行符来标识命令结束

            // 尝试向 sidecar 的 stdin 写入命令
            if let Err(err) = process.write(command.as_bytes()) {
                println!("[tauri] 向 sidecar stdin 写入失败: {}", err);
                // 如果关闭失败，恢复进程引用
                *child_process = Some(process);
                return Err(format!("向 sidecar stdin 写入失败: {}", err));
            }

            println!("[tauri] 已向 sidecar 发送 'sidecar shutdown' 命令。");
            Ok("'sidecar shutdown' 命令已发送。".to_string())
        } else {
            println!("[tauri] 没有活跃的 sidecar 进程可以关闭。");
            Err("没有活跃的 sidecar 进程可以关闭。".to_string())
        }
    } else {
        Err("未找到 Sidecar 进程状态。".to_string())
    }
}

// 定义启动 sidecar 进程的命令
#[tauri::command]
fn start_sidecar(app_handle: tauri::AppHandle) -> Result<String, String> {
    println!("[tauri] 收到启动 sidecar 的命令。");
    spawn_and_monitor_sidecar(app_handle)?;
    Ok("Sidecar 已启动并开始监控。".to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        // 添加必要的插件
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 在应用状态中存储初始的 sidecar 进程
            app.manage(Arc::new(Mutex::new(None::<CommandChild>)));
            // 克隆应用句柄供其他地方使用
            let app_handle = app.handle().clone();
            // 在启动时启动 Python sidecar
            println!("[tauri] 正在创建 sidecar...");
            spawn_and_monitor_sidecar(app_handle).ok();
            println!("[tauri] Sidecar 已启动并开始监控。");
            Ok(())
        })
        // 注册 shutdown_server 命令
        .invoke_handler(tauri::generate_handler![
            start_sidecar,
            shutdown_sidecar,
            toggle_fullscreen
        ])
        .build(tauri::generate_context!())
        .expect("运行 tauri 应用程序时出错")
        .run(|app_handle, event| match event {
            // 确保在应用关闭时杀死 Python sidecar
            RunEvent::ExitRequested { .. } => {
                if let Some(child_process) =
                    app_handle.try_state::<Arc<Mutex<Option<CommandChild>>>>()
                {
                    if let Ok(mut child) = child_process.lock() {
                        if let Some(process) = child.as_mut() {
                            // 通过 stdin 向 sidecar 发送消息，让它自行终止
                            let command = "sidecar shutdown\n";
                            let buf: &[u8] = command.as_bytes();
                            let _ = process.write(buf);

                            // *重要* `process.kill()` 只会关闭父 sidecar（python 进程）。Tauri 不知道由"引导加载器"脚本启动的第二个进程。
                            // 这只适用于使用 PyInstaller 编译"单文件"exe 的情况。否则，只需使用下面的代码行正常杀死进程。
                            // let _ = process.kill();

                            println!("[tauri] Sidecar 已关闭。");
                        }
                    }
                }
            }
            _ => {}
        });
}
