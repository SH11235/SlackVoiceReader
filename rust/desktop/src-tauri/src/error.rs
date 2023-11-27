use cpal::DeviceNameError;
use rodio::StreamError;
use std::fmt;
use tauri::InvokeError;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Json(serde_json::Error),
    DeviceName(DeviceNameError),
    Stream(StreamError),
    Anyhow(anyhow::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Json(e) => write!(f, "JSON error: {}", e),
            AppError::DeviceName(e) => write!(f, "Device name error: {}", e),
            AppError::Stream(e) => write!(f, "Stream error: {}", e),
            AppError::Anyhow(e) => write!(f, "Anyhow error: {}", e),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::Json(error)
    }
}

impl From<DeviceNameError> for AppError {
    fn from(error: DeviceNameError) -> Self {
        AppError::DeviceName(error)
    }
}

impl From<StreamError> for AppError {
    fn from(error: StreamError) -> Self {
        AppError::Stream(error)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::Anyhow(e)
    }
}

impl Into<InvokeError> for AppError {
    fn into(self) -> InvokeError {
        // Convert AppError into InvokeError here.
        // You may need to adjust this based on the specifics of your AppError and InvokeError types.
        InvokeError::from(self.to_string())
    }
}
