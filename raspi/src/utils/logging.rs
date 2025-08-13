use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::{io::stdout, sync::Arc};
use tokio::{
    io::{AsyncWriteExt, Stderr, Stdout},
    sync::Mutex,
};

pub fn clear_screen_and_return_to_zero() {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
}

#[derive(Clone)]
pub struct AsyncLogger {
    stdout_mutex: Arc<Mutex<Stdout>>,
    stderr_mutex: Arc<Mutex<Stderr>>,
}

impl AsyncLogger {
    pub fn new(stdout_mutex: Arc<Mutex<Stdout>>, stderr_mutex: Arc<Mutex<Stderr>>) -> AsyncLogger {
        AsyncLogger {
            stdout_mutex,
            stderr_mutex,
        }
    }

    pub async fn out_print(&self, msg: String) {
        Self::print_with_lock(&self.stdout_mutex, &msg).await;
    }

    pub async fn err_print(&self, msg: String) {
        Self::print_with_lock(&self.stderr_mutex, &msg).await;
    }

    async fn print_with_lock<T>(std_mutex: &Arc<Mutex<T>>, msg: &String)
    where
        T: AsyncWriteExt + Unpin + Send + 'static,
    {
        let mut stdout = std_mutex.lock().await;
        let mut msg_newline = msg.clone();
        msg_newline.push('\n');
        if let Err(e) = stdout.write_all(msg_newline.as_bytes()).await {
            eprintln!("Error writing to stdout {e:?}");
        }
    }
}
