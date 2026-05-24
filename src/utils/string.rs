/*
 *
 *  *
 *  *      Copyright (c) 2018-2025, SnackCloud All rights reserved.
 *  *
 *  *   Redistribution and use in source and binary forms, with or without
 *  *   modification, are permitted provided that the following conditions are met:
 *  *
 *  *   Redistributions of source code must retain the above copyright notice,
 *  *   this list of conditions and the following disclaimer.
 *  *   Redistributions in binary form must reproduce the above copyright
 *  *   notice, this list of conditions and the following disclaimer in the
 *  *   documentation and/or other materials provided with the distribution.
 *  *   Neither the name of the www.snackcloud.cn developer nor the names of its
 *  *   contributors may be used to endorse or promote products derived from
 *  *   this software without specific prior written permission.
 *  *   Author: SnackCloud
 *  *
 *  
 */

pub const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789";
/// 生成随机字符串
pub fn random_string(length: usize) -> String {
    use rand::Rng;
    
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..length);
            CHARSET[idx] as char
        })
        .collect()
}

/// 生成UUID
pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// 驼峰命名转蛇形命名
pub fn camel_to_snake(camel: &str) -> String {
    let mut snake = String::new();
    let mut chars = camel.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_uppercase() && !snake.is_empty() {
            snake.push('_');
        }
        snake.push(c.to_lowercase().next().unwrap());
    }

    snake
}

/// 蛇形命名转驼峰命名
pub fn snake_to_camel(snake: &str) -> String {
    let mut camel = String::new();
    let mut next_upper = false;

    for c in snake.chars() {
        if c == '_' {
            next_upper = true;
        } else if next_upper {
            camel.push(c.to_uppercase().next().unwrap());
            next_upper = false;
        } else {
            camel.push(c);
        }
    }

    camel
}

/// 截断字符串
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
