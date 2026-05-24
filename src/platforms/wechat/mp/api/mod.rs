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

mod customservice;
mod qrcode;
mod user;
mod menu;
mod oauth2;
mod media;
mod template_msg;
mod subscribe_msg;
mod ocr;
mod member;
mod card;
mod mass_message;
mod autoreply;
mod invoice;
mod comment;
mod publish;
mod store;
mod medical;
mod onecode;
mod ai;
mod image;
mod api_manage;

pub use self::oauth2::*;
pub use self::qrcode::*;
pub use self::user::*;
pub use self::customservice::*;
pub use self::template_msg::*;
pub use self::subscribe_msg::*;
pub use self::media::*;
pub use self::menu::*;
pub use self::ocr::*;
pub use self::member::*;
pub use self::card::*;
pub use self::mass_message::*;
pub use self::autoreply::*;
pub use self::invoice::*;
pub use self::comment::*;
pub use self::publish::*;
pub use self::store::*;
pub use self::medical::*;
pub use self::onecode::*;
pub use self::ai::*;
pub use self::image::*;
pub use self::api_manage::*;


