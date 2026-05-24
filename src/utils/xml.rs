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

//! XML工具

use crate::errors::{LabraError, LabradorResult};
use quick_xml::events::BytesCData;
use quick_xml::de;
use serde::{Deserialize, Serialize};


#[derive(Serialize)]
#[serde(rename = "xml")]
pub struct XmlMap<'a, T>(pub &'a T);


/// XML序列化器
pub struct XmlSerializer;

impl XmlSerializer {
    /// 序列化结构体为XML字符串
    pub fn serialize<T: Serialize>(value: &T) -> LabradorResult<String> {
        // 创建字符串缓冲区
        let mut xml_string = String::new();
        // 创建序列化器，引用这个缓冲区
        let mut writer = quick_xml::se::Serializer::new(&mut xml_string);

        // 配置序列化器
        writer.indent(' ', 2);  // 使用2个空格缩进
        // 其他可能的配置：
        // writer.escape(false);  // 禁用转义
        // writer.text_indent(' ', 4);  // 文本缩进
        // 执行序列化
        value.serialize(writer)
            .map_err(|e| LabraError::Xml(format!("序列化失败: {}", e)))?;

        // 返回缓冲区中的内容
        Ok(xml_string)
    }

    /// 序列化结构体为XML字符串（带根元素）
    pub fn serialize_with_root<T: Serialize>(value: &T, root_name: &str) -> LabradorResult<String> {
        let xml = Self::serialize(value)?;
        Ok(format!("<{}>{}</{}>", root_name, xml, root_name))
    }

    /// 反序列化XML字符串为结构体
    pub fn deserialize<T: for<'de> Deserialize<'de>>(xml: &str) -> LabradorResult<T> {
        de::from_str(xml)
            .map_err(|e| LabraError::Xml(format!("反序列化失败: {}", e)))
    }

    /// 反序列化带根元素的XML字符串
    pub fn deserialize_with_root<T: for<'de> Deserialize<'de>>(xml: &str, root_name: &str) -> LabradorResult<T> {
        let wrapped_xml = format!("<{}>{}</{}>", root_name, xml, root_name);
        Self::deserialize(&wrapped_xml)
    }

    /// 美化XML字符串
    pub fn pretty_print(xml: &str) -> LabradorResult<String> {
        use quick_xml::events::Event;
        use quick_xml::reader::Reader;
        use quick_xml::writer::Writer;

        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);

        loop {
            match reader.read_event() {
                Ok(Event::Eof) => break,
                Ok(event) => {
                    writer.write_event(event)
                        .map_err(|e| LabraError::Xml(format!("格式化失败: {}", e)))?;
                }
                Err(e) => return Err(LabraError::Xml(format!("解析失败: {}", e))),
            }
        }

        String::from_utf8(writer.into_inner())
            .map_err(|e| LabraError::Xml(format!("UTF-8转换失败: {}", e)))
    }

    /// 压缩XML字符串（移除空白和换行）
    pub fn minify(xml: &str) -> LabradorResult<String> {
        use quick_xml::events::Event;
        use quick_xml::reader::Reader;
        use quick_xml::writer::Writer;

        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut writer = Writer::new(Vec::new());

        loop {
            match reader.read_event() {
                Ok(Event::Eof) => break,
                Ok(event) => {
                    writer.write_event(event)
                        .map_err(|e| LabraError::Xml(format!("压缩失败: {}", e)))?;
                }
                Err(e) => return Err(LabraError::Xml(format!("解析失败: {}", e))),
            }
        }

        String::from_utf8(writer.into_inner())
            .map_err(|e| LabraError::Xml(format!("UTF-8转换失败: {}", e)))
    }

    /// 验证XML格式
    pub fn validate(xml: &str) -> LabradorResult<()> {
        use quick_xml::events::Event;
        use quick_xml::reader::Reader;

        let mut reader = Reader::from_str(xml);
        reader.config_mut().check_end_names = true;

        loop {
            match reader.read_event() {
                Ok(Event::Eof) => break,
                Ok(_) => continue,
                Err(e) => return Err(LabraError::Xml(format!("验证失败: {}", e))),
            }
        }

        Ok(())
    }

    /// 提取XML中的CDATA内容
    pub fn extract_cdata(xml: &str) -> LabradorResult<Vec<String>> {
        use quick_xml::events::Event;
        use quick_xml::reader::Reader;

        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(false);

        let mut cdata_contents = Vec::new();

        loop {
            match reader.read_event() {
                Ok(Event::Eof) => break,
                Ok(Event::CData(cdata)) => {
                    if let Ok(text) = String::from_utf8(cdata.to_vec()) {
                        cdata_contents.push(text);
                    }
                }
                Ok(_) => continue,
                Err(e) => return Err(LabraError::Xml(format!("解析失败: {}", e))),
            }
        }

        Ok(cdata_contents)
    }

    /// 提取XML中的文本内容
    pub fn extract_text(xml: &str) -> LabradorResult<Vec<String>> {
        use quick_xml::events::Event;
        use quick_xml::reader::Reader;

        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut text_contents = Vec::new();

        loop {
            match reader.read_event() {
                Ok(Event::Eof) => break,
                Ok(Event::Text(text)) => {
                    if let Ok(text_str) = text.decode() {
                        text_contents.push(text_str.into_owned());
                    }
                }
                Ok(_) => continue,
                Err(e) => return Err(LabraError::Xml(format!("解析失败: {}", e))),
            }
        }

        Ok(text_contents)
    }
}

/// XML解析器
pub struct XmlParser;

impl XmlParser {
    /// 解析XML为JSON Value
    pub fn to_json(xml: &str) -> LabradorResult<serde_json::Value> {
        let mut reader = quick_xml::reader::Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut json = serde_json::Map::new();
        Self::parse_element(&mut reader, &mut json)?;

        Ok(serde_json::Value::Object(json))
    }

    fn parse_element(
        reader: &mut quick_xml::reader::Reader<&[u8]>,
        parent: &mut serde_json::Map<String, serde_json::Value>,
    ) -> LabradorResult<()> {
        use quick_xml::events::Event;

        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) => {
                    let name = String::from_utf8_lossy(e.name().into_inner());
                    let mut attributes = serde_json::Map::new();
                    let mut children = serde_json::Map::new();

                    // 解析属性
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            let key = String::from_utf8_lossy(attr.key.as_ref());
                            let value = String::from_utf8_lossy(&attr.value).into_owned();
                            attributes.insert(key.to_string(), serde_json::Value::String(value));
                        }
                    }

                    // 解析子元素
                    Self::parse_element(reader, &mut children)?;

                    // 创建元素对象
                    let mut element = serde_json::Map::new();
                    if !attributes.is_empty() {
                        element.insert("@attributes".to_string(), serde_json::Value::Object(attributes));
                    }
                    if !children.is_empty() {
                        element.insert("@children".to_string(), serde_json::Value::Object(children));
                    }

                    parent.insert(name.to_string(), serde_json::Value::Object(element));
                }
                Ok(Event::End(_)) => break,
                Ok(Event::Text(e)) => {
                    let text = e.decode()
                        .map_err(|e| LabraError::Xml(format!("文本解码失败: {}", e)))?
                        .into_owned();
                    if !text.trim().is_empty() {
                        parent.insert("@text".to_string(), serde_json::Value::String(text));
                    }
                }
                Ok(Event::CData(e)) => {
                    let text = String::from_utf8(e.to_vec())
                        .map_err(|e| LabraError::Xml(format!("CDATA解码失败: {}", e)))?;
                    parent.insert("@cdata".to_string(), serde_json::Value::String(text));
                }
                Ok(Event::Eof) => break,
                Ok(_) => continue,
                Err(e) => return Err(LabraError::Xml(format!("解析失败: {}", e))),
            }
        }

        Ok(())
    }

    /// 从JSON Value生成XML
    pub fn from_json(json: &serde_json::Value) -> LabradorResult<String> {
        let mut writer = quick_xml::writer::Writer::new_with_indent(Vec::new(), b' ', 2);

        Self::write_element(&mut writer, "root", json)?;

        String::from_utf8(writer.into_inner())
            .map_err(|e| LabraError::Xml(format!("UTF-8转换失败: {}", e)))
    }

    fn write_element(
        writer: &mut quick_xml::writer::Writer<Vec<u8>>,
        name: &str,
        value: &serde_json::Value,
    ) -> LabradorResult<()> {
        use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

        match value {
            serde_json::Value::Object(obj) => {
                let mut elem = BytesStart::new(name);

                // 处理属性
                if let Some(attrs) = obj.get("@attributes") {
                    if let serde_json::Value::Object(attr_map) = attrs {
                        for (key, val) in attr_map {
                            if let serde_json::Value::String(s) = val {
                                elem.push_attribute((key.as_str(), s.as_str()));
                            }
                        }
                    }
                }

                writer.write_event(Event::Start(elem))
                    .map_err(|e| LabraError::Xml(format!("写入开始标签失败: {}", e)))?;

                // 处理文本内容
                if let Some(text) = obj.get("@text") {
                    if let serde_json::Value::String(s) = text {
                        writer.write_event(Event::Text(BytesText::new(s)))
                            .map_err(|e| LabraError::Xml(format!("写入文本失败: {}", e)))?;
                    }
                }

                // 处理CDATA
                if let Some(cdata) = obj.get("@cdata") {
                    if let serde_json::Value::String(s) = cdata {
                        writer.write_event(Event::CData(BytesCData::new(s)))
                            .map_err(|e| LabraError::Xml(format!("写入CDATA失败: {}", e)))?;
                    }
                }

                // 处理子元素
                if let Some(children) = obj.get("@children") {
                    if let serde_json::Value::Object(child_map) = children {
                        for (key, val) in child_map {
                            Self::write_element(writer, key, val)?;
                        }
                    }
                }

                writer.write_event(Event::End(BytesEnd::new(name)))
                    .map_err(|e| LabraError::Xml(format!("写入结束标签失败: {}", e)))?;
            }
            serde_json::Value::String(s) => {
                let elem = BytesStart::new(name);
                writer.write_event(Event::Start(elem))
                    .map_err(|e| LabraError::Xml(format!("写入开始标签失败: {}", e)))?;

                writer.write_event(Event::Text(BytesText::new(s)))
                    .map_err(|e| LabraError::Xml(format!("写入文本失败: {}", e)))?;

                writer.write_event(Event::End(BytesEnd::new(name)))
                    .map_err(|e| LabraError::Xml(format!("写入结束标签失败: {}", e)))?;
            }
            serde_json::Value::Number(n) => {
                let elem = BytesStart::new(name);
                writer.write_event(Event::Start(elem))
                    .map_err(|e| LabraError::Xml(format!("写入开始标签失败: {}", e)))?;

                writer.write_event(Event::Text(BytesText::new(&n.to_string())))
                    .map_err(|e| LabraError::Xml(format!("写入文本失败: {}", e)))?;

                writer.write_event(Event::End(BytesEnd::new(name)))
                    .map_err(|e| LabraError::Xml(format!("写入结束标签失败: {}", e)))?;
            }
            serde_json::Value::Bool(b) => {
                let elem = BytesStart::new(name);
                writer.write_event(Event::Start(elem))
                    .map_err(|e| LabraError::Xml(format!("写入开始标签失败: {}", e)))?;

                writer.write_event(Event::Text(BytesText::new(&b.to_string())))
                    .map_err(|e| LabraError::Xml(format!("写入文本失败: {}", e)))?;

                writer.write_event(Event::End(BytesEnd::new(name)))
                    .map_err(|e| LabraError::Xml(format!("写入结束标签失败: {}", e)))?;
            }
            serde_json::Value::Null => {
                let elem = BytesStart::new(name);
                writer.write_event(Event::Empty(elem))
                    .map_err(|e| LabraError::Xml(format!("写入空标签失败: {}", e)))?;
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    Self::write_element(writer, name, item)?;
                }
            }
        }

        Ok(())
    }

    /// 查询XML节点
    pub fn query(xml: &str, xpath: &str) -> LabradorResult<Vec<String>> {
        // 简化实现，实际应该使用xpath库
        let json = Self::to_json(xml)?;
        Self::query_json(&json, xpath)
    }

    fn query_json(json: &serde_json::Value, path: &str) -> LabradorResult<Vec<String>> {
        let mut results = Vec::new();
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

        if parts.is_empty() {
            results.push(json.to_string());
        } else {
            Self::query_recursive(json, &parts, 0, &mut results)?;
        }

        Ok(results)
    }

    fn query_recursive(
        value: &serde_json::Value,
        parts: &[&str],
        index: usize,
        results: &mut Vec<String>,
    ) -> LabradorResult<()> {
        if index >= parts.len() {
            results.push(value.to_string());
            return Ok(());
        }

        let part = parts[index];

        match value {
            serde_json::Value::Object(obj) => {
                if let Some(child) = obj.get(part) {
                    Self::query_recursive(child, parts, index + 1, results)?;
                }
                // 也检查@children
                if let Some(children) = obj.get("@children") {
                    if let serde_json::Value::Object(child_map) = children {
                        if let Some(child) = child_map.get(part) {
                            Self::query_recursive(child, parts, index + 1, results)?;
                        }
                    }
                }
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    Self::query_recursive(item, parts, index, results)?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

/// XML构建器
pub struct XmlBuilder {
    elements: Vec<XmlElement>,
}

impl XmlBuilder {
    /// 创建新的XML构建器
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    /// 添加元素
    pub fn element(mut self, name: &str) -> XmlElementBuilder {
        let element = XmlElement::new(name);
        self.elements.push(element);
        let len = self.elements.len();
        XmlElementBuilder {
            builder: self,
            index:  len - 1,
        }
    }

    /// 构建XML字符串
    pub fn build(self) -> LabradorResult<String> {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");

        for element in self.elements {
            xml.push_str(&element.to_string());
            xml.push('\n');
        }

        Ok(xml)
    }
}

/// XML元素
#[derive(Clone)]
pub struct XmlElement {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<XmlElement>,
    text: Option<String>,
    cdata: Option<String>,
}

impl XmlElement {
    /// 创建新的XML元素
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attributes: Vec::new(),
            children: Vec::new(),
            text: None,
            cdata: None,
        }
    }

    /// 添加属性
    pub fn attr(mut self, key: &str, value: &str) -> Self {
        self.attributes.push((key.to_string(), value.to_string()));
        self
    }

    /// 设置文本内容
    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    /// 设置CDATA内容
    pub fn cdata(mut self, cdata: &str) -> Self {
        self.cdata = Some(cdata.to_string());
        self
    }

    /// 添加子元素
    pub fn child(mut self, child: XmlElement) -> Self {
        self.children.push(child);
        self
    }

    /// 转换为字符串
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        self.write_to_string(&mut result, 0);
        result
    }

    fn write_to_string(&self, output: &mut String, indent: usize) {
        // 缩进
        for _ in 0..indent {
            output.push_str("  ");
        }

        // 开始标签
        output.push('<');
        output.push_str(&self.name);

        // 属性
        for (key, value) in &self.attributes {
            output.push(' ');
            output.push_str(key);
            output.push_str("=\"");
            output.push_str(&Self::escape_attribute(value));
            output.push('"');
        }

        // 检查是否有内容
        let has_content = self.text.is_some() || self.cdata.is_some() || !self.children.is_empty();

        if !has_content {
            // 空元素
            output.push_str("/>");
            return;
        }

        // 结束开始标签
        output.push('>');

        // 文本内容
        if let Some(text) = &self.text {
            output.push_str(&Self::escape_text(text));
        }

        // CDATA内容
        if let Some(cdata) = &self.cdata {
            output.push_str("<![CDATA[");
            output.push_str(cdata);
            output.push_str("]]>");
        }

        // 子元素
        if !self.children.is_empty() {
            output.push('\n');
            for child in &self.children {
                child.write_to_string(output, indent + 1);
                output.push('\n');
            }
            for _ in 0..indent {
                output.push_str("  ");
            }
        }

        // 结束标签
        output.push_str("</");
        output.push_str(&self.name);
        output.push('>');
    }

    fn escape_text(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }

    fn escape_attribute(attr: &str) -> String {
        Self::escape_text(attr)
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}

/// XML元素构建器
pub struct XmlElementBuilder {
    builder: XmlBuilder,
    index: usize,
}

impl XmlElementBuilder {
    /// 添加属性
    pub fn attr(mut self, key: &str, value: &str) -> Self {
        let element = self.builder.elements[self.index].clone();
        self.builder.elements[self.index] = element
            .attr(key, value);
        self
    }

    /// 设置文本内容
    pub fn text(mut self, text: &str) -> Self {
        let element = self.builder.elements[self.index].clone();
        self.builder.elements[self.index] = element
            .text(text);
        self
    }

    /// 设置CDATA内容
    pub fn cdata(mut self, cdata: &str) -> Self {
        self.builder.elements[self.index] = self.builder.elements[self.index]
            .clone()
            .cdata(cdata);
        self
    }

    /// 添加子元素
    pub fn child(mut self, child: XmlElement) -> Self {
        self.builder.elements[self.index] = self.builder.elements[self.index]
            .clone()
            .child(child);
        self
    }

    /// 完成当前元素，返回构建器
    pub fn end(self) -> XmlBuilder {
        self.builder
    }
}

impl Default for XmlBuilder {
    fn default() -> Self {
        Self::new()
    }
}