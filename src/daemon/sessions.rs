use std::sync::MutexGuard;

pub type LockedSessions<'a> = MutexGuard<'a, Vec<libcommand::Session>>;

pub fn get_sessions_lock() -> Result<LockedSessions<'static>, String> {
    super::SESSIONS.lock()
        .map_err(|_| format!("Failed to get Mutex lock of sessions manager"))
}

pub fn remove_session_by_id(lock: &mut LockedSessions<'static>, id: &str) -> Result<(), String> {
    let position = lock.iter().position(|s| s.id == id)
        .ok_or(format!("Session id not found"))?;
    lock.remove(position);
    Ok(())
}