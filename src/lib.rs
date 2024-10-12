#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr, CString};

pub struct Conn {
    conn: *mut xdb_conn_t,
}

pub struct XdbResult {
    res: *mut xdb_res_t,
}

impl XdbResult {
    pub fn column_count(&self) -> u32 {
        unsafe { (*self.res).col_count.into() }
    }
}

impl Conn {
    pub fn open(db_path: &str) -> Result<Self, String> {
        let c_path = CString::new(db_path).unwrap();
        let conn = unsafe { xdb_open(c_path.as_ptr()) };
        if conn.is_null() {
            Err("Failed to open database".to_string())
        } else {
            Ok(Conn { conn })
        }
    }

    pub fn exec(&self, sql: &str) -> Result<XdbResult, String> {
        let c_sql = CString::new(sql).unwrap();
        let res = unsafe { xdb_exec(self.conn, c_sql.as_ptr()) };
        if res.is_null() {
            return Err("Failed to execute SQL".to_string());
        }
        let result = XdbResult { res };
        unsafe {
            if (*result.res).errcode != 0 {
                let err_msg = CStr::from_ptr(xdb_errmsg(result.res))
                    .to_string_lossy()
                    .into_owned();
                xdb_free_result(result.res);
                return Err(format!("SQL error {}: {}", (*result.res).errcode, err_msg));
            }
        }
        Ok(result)
    }

    pub fn begin(&self) -> Result<(), String> {
        let ret = unsafe { xdb_begin(self.conn) };
        if ret != 0 {
            Err("Failed to begin transaction".to_string())
        } else {
            Ok(())
        }
    }

    pub fn commit(&self) -> Result<(), String> {
        let ret = unsafe { xdb_commit(self.conn) };
        if ret != 0 {
            Err("Failed to commit transaction".to_string())
        } else {
            Ok(())
        }
    }

    pub fn rollback(&self) -> Result<(), String> {
        let ret = unsafe { xdb_rollback(self.conn) };
        if ret != 0 {
            Err("Failed to rollback transaction".to_string())
        } else {
            Ok(())
        }
    }
}

impl Drop for Conn {
    fn drop(&mut self) {
        unsafe { xdb_close(self.conn) }
    }
}

impl XdbResult {
    pub fn fetch_row(&self) -> Option<Vec<Option<String>>> {
        let row = unsafe { xdb_fetch_row(self.res) };
        if row.is_null() {
            return None;
        }

        let col_count = unsafe { (*self.res).col_count };
        let mut result = Vec::with_capacity(col_count as usize);

        for i in 0..col_count {
            let value = unsafe {
                // Try to get the value as a string first
                let c_str = xdb_column_str((*self.res).col_meta, row, i as u16);
                if !c_str.is_null() {
                    Some(CStr::from_ptr(c_str).to_string_lossy().into_owned())
                } else {
                    // If string fails, try to get it as an integer
                    let int_val = xdb_column_int((*self.res).col_meta, row, i as u16);
                    Some(int_val.to_string())
                }
            };
            result.push(value);
        }

        Some(result)
    }
}

impl Drop for XdbResult {
    fn drop(&mut self) {
        unsafe { xdb_free_result(self.res) }
    }
}

pub fn version() -> String {
    unsafe {
        CStr::from_ptr(xdb_version())
            .to_string_lossy()
            .into_owned()
    }
}

// Implement Send and Sync for Conn and XdbResult
unsafe impl Send for Conn {}
unsafe impl Sync for Conn {}
unsafe impl Send for XdbResult {}
unsafe impl Sync for XdbResult {}
