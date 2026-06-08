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

mod ai;
mod api_manage;
mod autoreply;
mod card;
mod comment;
mod customservice;
mod image;
mod invoice;
mod mass_message;
mod media;
mod medical;
mod member;
mod menu;
mod oauth2;
mod ocr;
mod onecode;
mod publish;
mod qrcode;
mod store;
mod subscribe_msg;
mod template_msg;
mod user;

pub use self::ai::*;
pub use self::api_manage::*;
pub use self::autoreply::*;
pub use self::card::*;
pub use self::comment::*;
pub use self::customservice::*;
pub use self::image::*;
pub use self::invoice::*;
pub use self::mass_message::*;
pub use self::media::*;
pub use self::medical::*;
pub use self::member::*;
pub use self::menu::*;
pub use self::oauth2::*;
pub use self::ocr::*;
pub use self::onecode::*;
pub use self::publish::*;
pub use self::qrcode::*;
pub use self::store::*;
pub use self::subscribe_msg::*;
pub use self::template_msg::*;
pub use self::user::*;
