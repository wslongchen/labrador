/*
 *
 *  *
 *  *      Copyright (c) 2018-2025, WoofCloud All rights reserved.
 *  *
 *  *   Redistribution and use in source and binary forms, with or without
 *  *   modification, are permitted provided that the following conditions are met:
 *  *
 *  *   Redistributions of source code must retain the above copyright notice,
 *  *   this list of conditions and the following disclaimer.
 *  *   Redistributions in binary form must reproduce the above copyright
 *  *   notice, this list of conditions and the following disclaimer in the
 *  *   documentation and/or other materials provided with the distribution.
 *  *   Neither the name of the www.woofcloud.com developer nor the names of its
 *  *   contributors may be used to endorse or promote products derived from
 *  *   this software without specific prior written permission.
 *  *   Author: WoofCloud
 *  *
 *
 */
use crate::errors::LabradorResult;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// 读取文件内容
pub fn read_file(path: impl AsRef<Path>) -> LabradorResult<Vec<u8>> {
    fs::read(path).map_err(crate::errors::LabraError::Io)
}

/// 读取文件内容并返回文件名和字节数据
pub fn read_file_with_name(file_path: &str) -> LabradorResult<(String, Vec<u8>)> {
    let path = Path::new(file_path);
    let file_name = path
        .file_name()
        .and_then(|v| v.to_str())
        .unwrap_or("file")
        .to_string();
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok((file_name, content))
}

/// 读取文本文件
pub fn read_text_file(path: impl AsRef<Path>) -> LabradorResult<String> {
    fs::read_to_string(path).map_err(crate::errors::LabraError::Io)
}

/// 写入文件
pub fn write_file(path: impl AsRef<Path>, data: &[u8]) -> LabradorResult<()> {
    fs::write(path, data).map_err(crate::errors::LabraError::Io)
}

/// 写入文本文件
pub fn write_text_file(path: impl AsRef<Path>, text: &str) -> LabradorResult<()> {
    fs::write(path, text).map_err(crate::errors::LabraError::Io)
}

/// 检查文件是否存在
pub fn file_exists(path: impl AsRef<Path>) -> bool {
    Path::new(path.as_ref()).exists()
}

/// 获取文件大小
pub fn file_size(path: impl AsRef<Path>) -> LabradorResult<u64> {
    fs::metadata(path)
        .map(|meta| meta.len())
        .map_err(crate::errors::LabraError::Io)
}

/// 获取文件扩展名
pub fn file_extension(path: impl AsRef<Path>) -> Option<String> {
    path.as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

/// 创建目录
pub fn create_dir(path: impl AsRef<Path>) -> LabradorResult<()> {
    fs::create_dir_all(path).map_err(crate::errors::LabraError::Io)
}
