use crate::types::*;
use crate::value::Value;
use dtoa;
use itoa;
use std::str;
use rustils;
use std::collections::hash_map::HashMap;

fn value_bin_conversion(t: u16, ot: u16, data: Vec<u8>) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != BIN {
        return Err(format!(
            "Source value is not BIN but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        STRING => {
            let str_val = match str::from_utf8(&data) {
                Ok(str_val) => str_val,
                Err(err) => return Err(format!("Can not convert BIN to STR to {:?}: {}", &t, err).into()),
            };
            return Result::Ok(Value::from_string(str_val));
        }
        LIST => {
            let mut res = Value::list();
            for v in data.into_iter() {
                res = res.push(Value::from_bin(vec![v]));
            }
            return Result::Ok(res);
        }
        _ => Err(format!("Can not convert NODATA to {:?}", &t).into()),
    }
}

fn value_nodata_conversion(t: u16, ot: u16) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != NODATA {
        return Err(format!(
            "Source value is not NODATA but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        STRING => {
            return Result::Ok(Value::from_string("NODATA"));
        }
        LIST => {
            return Result::Ok(Value::from(vec![Value::nodata()]).unwrap());
        }
        _ => Err(format!("Can not convert NODATA to {:?}", &t).into()),
    }
}

fn value_none_conversion(t: u16, ot: u16) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != NONE {
        return Err(format!(
            "Source value is not NONE but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        STRING => {
            return Result::Ok(Value::from_string("NONE"));
        }
        LIST => {
            return Result::Ok(Value::from(vec![Value::none()]).unwrap());
        }
        _ => Err(format!("Can not convert NONE to {:?}", &t).into()),
    }
}

fn value_call_conversion(t: u16, ot: u16, name: String) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != CALL {
        return Err(format!(
            "Source value is not CALL but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        STRING => {
            return Result::Ok(Value::from_string(format!("F({})", &name)));
        }
        _ => Err(format!("Can not convert CALL to {:?}", &t).into()),
    }
}

fn value_ptr_conversion(t: u16, ot: u16, name: String) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != PTR {
        return Err(format!(
            "Source value is not PTR but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        STRING => {
            return Result::Ok(Value::from_string(format!("`({})", &name)));
        }
        _ => Err(format!("Can not convert PTR to {:?}", &t).into()),
    }
}

fn value_float_conversion(t: u16, ot: u16, val: f64) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != FLOAT {
        return Err(format!(
            "Source value is not FLOAT but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        FLOAT => {
            return Result::Ok(Value::from(val as f64).unwrap());
        }
        INTEGER => {
            return Result::Ok(Value::from(val as i64).unwrap());
        }
        BOOL => {
            if val == 0.0 as f64 {
                return Result::Ok(Value::from(false).unwrap());
            }
            return Result::Ok(Value::from(true).unwrap());
        }
        STRING => {
            let mut buffer = dtoa::Buffer::new();
            return Result::Ok(Value::from_string(buffer.format(val).to_string()));
        }
        TEXTBUFFER => {
            let mut buffer = dtoa::Buffer::new();
            return Result::Ok(Value::text_buffer(buffer.format(val).to_string()));
        }
        LIST => {
            return Result::Ok(Value::from(vec![Value::from(val as f64).unwrap()]).unwrap());
        }
        _ => Err(format!("Can not convert float to {:?}", &t).into()),
    }
}

fn value_integer_conversion(
    t: u16,
    ot: u16,
    val: i64,
) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != INTEGER {
        return Err(format!(
            "Source value is not INTEGER but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        FLOAT => {
            return Result::Ok(Value::from(val as f64).unwrap());
        }
        INTEGER => {
            return Result::Ok(Value::from(val as i64).unwrap());
        }
        BOOL => {
            if val == 0 as i64 {
                return Result::Ok(Value::from(false).unwrap());
            }
            return Result::Ok(Value::from(true).unwrap());
        }
        STRING => {
            let mut buffer = itoa::Buffer::new();
            return Result::Ok(Value::from_string(buffer.format(val).to_string()));
        }
        TEXTBUFFER => {
            let mut buffer = itoa::Buffer::new();
            return Result::Ok(Value::text_buffer(buffer.format(val).to_string()));
        }
        LIST => {
            return Result::Ok(Value::from(vec![Value::from(val as i64).unwrap()]).unwrap());
        }
        _ => Err(format!("Can not convert integer to {:?}", &t).into()),
    }
}

fn value_string_conversion(
    t: u16,
    ot: u16,
    val: &String,
) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != STRING && ot != TEXTBUFFER {
        return Err(format!(
            "Source value is not STRING but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        FLOAT => match rustils::parse::double::string_to_f64_res(val.to_string()) {
            Ok(f_res) => {
                return Result::Ok(Value::from(f_res as f64).unwrap());
            }
            Err(err) => Err(format!("Can not convert string to float {:?}", err).into()),
        },
        INTEGER => match rustils::parse::long::string_to_i64_res(val.to_string()) {
            Ok(i_res) => {
                return Result::Ok(Value::from(i_res as i64).unwrap());
            }
            Err(err) => Err(format!("Can not convert string to integer {:?}", err).into()),
        },
        BOOL => {
            return Result::Ok(Value::from_bool(rustils::parse::boolean::string_to_bool(
                val.to_string(),
            )));
        }
        STRING => {
            return Result::Ok(Value::from_string(val.to_string()));
        }
        TEXTBUFFER => {
            return Result::Ok(Value::text_buffer(val.to_string()));
        }
        BIN => {
            let bytes: Vec<u8> = val.clone().into_bytes();
            return Result::Ok(Value::from_bin(bytes));
        }
        LIST => {
            return Result::Ok(Value::from(vec![Value::from(val.to_string()).unwrap()]).unwrap());
        }
        _ => Err(format!("Can not convert string to {:?}", &t).into()),
    }
}

fn value_bool_conversion(t: u16, ot: u16, val: bool) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != BOOL {
        return Err(format!(
            "Source value is not BOOL but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        FLOAT => {
            if val {
                return Result::Ok(Value::from_float(1.0));
            } else {
                return Result::Ok(Value::from_float(0.0));
            }
        }
        INTEGER => {
            if val {
                return Result::Ok(Value::from_int(1));
            } else {
                return Result::Ok(Value::from_int(0));
            }
        }
        BOOL => {
            return Result::Ok(Value::from_bool(val));
        }
        STRING => {
            return Result::Ok(Value::from_string(format!("{}", val)));
        }
        TEXTBUFFER => {
            return Result::Ok(Value::text_buffer(format!("{}", val)));
        }
        LIST => {
            return Result::Ok(Value::from(vec![Value::from_bool(val)]).unwrap());
        }
        _ => Err(format!("Can not convert bool to {:?}", &t).into()),
    }
}

fn value_matrix_conversion(
    t: u16,
    ot: u16,
    val: &Vec<Vec<Value>>,
) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != MATRIX  {
        return Err(format!(
            "Source value is not LIST but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        LIST => {
            let mut res: Vec<Value> = Vec::new();
            for v in val {
                let row = Value::from_list(v.to_vec());
                res.push(row);
            }
            return Result::Ok(Value::from_list(res));
        }
        _ => Err(format!("Can not convert list to {:?}", &t).into()),
    }
}

fn value_list_conversion(
    t: u16,
    ot: u16,
    val: &Vec<Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != LIST && ot != RESULT {
        return Err(format!(
            "Source value is not LIST but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        LIST => {
            return Result::Ok(Value::from_list(val.to_vec()));
        }
        RESULT => {
            return Result::Ok(Value::to_result(val.to_vec()));
        }
        QUEUE => {
            return Result::Ok(Value::to_queue(val.to_vec()));
        }
        FIFO => {
            return Result::Ok(Value::to_fifo(val.to_vec()));
        }
        FLOAT => {
            return Result::Ok(Value::from_float(val.len() as f64));
        }
        INTEGER => {
            return Result::Ok(Value::from_int(val.len() as i64));
        }
        BOOL => {
            if val.len() == 0 {
                return Result::Ok(Value::from_bool(false));
            } else {
                return Result::Ok(Value::from_bool(true));
            }
        }
        MAP => {
            let mut res: HashMap<String, Value> = HashMap::new();
            let mut c: u64 = 0;
            for v in val {
                res.insert(format!("{}", &c), v.clone());
                c += 1;
            }
            return Result::Ok(Value::from_dict(res));
        }
        STRING | TEXTBUFFER => {
            let mut out: String = "[".to_string();
            for v in val {
                match v.conv(STRING) {
                    Ok(s_v) => {
                        out = out + &" ".to_string();
                        out = out + &s_v.cast_string().unwrap();
                        out = out + &" :: ".to_string();
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
            out = out + &"]".to_string();
            if t == STRING {
                return Result::Ok(Value::from_string(out));
            } else {
                return Result::Ok(Value::text_buffer(out));
            }
        }
        MATRIX => {
            let mut res: Vec<Vec<Value>> = Vec::new();
            for r in val {
                let row_val = match r.conv(LIST) {
                    Ok(row) => row,
                    Err(err) => {
                        return Err(format!("Error converting row into matrix {}", err).into());
                    }
                };
                let row = match row_val.cast_list() {
                    Ok(row) => row,
                    Err(err) => {
                        return Err(format!("Error casting row into matrix {}", err).into());
                    }
                };
                res.push(row);
            }
            return Result::Ok(Value::from_matrix(res));
        }
        _ => Err(format!("Can not convert list to {:?}", &t).into()),
    }
}

fn value_queue_conversion(
    t: u16,
    ot: u16,
    val: &Vec<Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut st = "queue";
    if ot != QUEUE && ot != FIFO {
        return Err(format!(
            "Source value is not QUEUE/FIFO but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    if ot == FIFO {
        st = "fifo";
    }
    match t {
        LIST => {
            return Result::Ok(Value::from_list(val.to_vec()));
        }
        RESULT => {
            return Result::Ok(Value::to_result(val.to_vec()));
        }
        QUEUE => {
            return Result::Ok(Value::to_queue(val.to_vec()));
        }
        FIFO => {
            return Result::Ok(Value::to_fifo(val.to_vec()));
        }
        FLOAT => {
            return Result::Ok(Value::from_float(val.len() as f64));
        }
        INTEGER => {
            return Result::Ok(Value::from_int(val.len() as i64));
        }
        BOOL => {
            if val.len() == 0 {
                return Result::Ok(Value::from_bool(false));
            } else {
                return Result::Ok(Value::from_bool(true));
            }
        }
        MAP => {
            let mut res: HashMap<String, Value> = HashMap::new();
            let mut c: u64 = 0;
            for v in val {
                res.insert(format!("{}", &c), v.clone());
                c += 1;
            }
            return Result::Ok(Value::from_dict(res));
        }
        STRING | TEXTBUFFER => {
            let mut out: String = format!("{}[", st).to_string();
            for v in val {
                match v.conv(STRING) {
                    Ok(s_v) => {
                        out = out + &" ".to_string();
                        out = out + &s_v.cast_string().unwrap();
                        out = out + &" :: ".to_string();
                    }
                    Err(_) => continue,
                }
            }
            out = out + &"]".to_string();
            if t == STRING {
                return Result::Ok(Value::from_string(out));
            } else {
                return Result::Ok(Value::text_buffer(out));
            }
        }
        _ => Err(format!("Can not convert list to {:?}", &t).into()),
    }
}

fn value_lambda_conversion(
    t: u16,
    ot: u16,
    val: &Vec<Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != LAMBDA {
        return Err(format!(
            "Source value is not LAMBDA but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        LIST => {
            return Result::Ok(Value::from_list(val.to_vec()));
        }
        LAMBDA => {
            return Result::Ok(Value::to_lambda(val.to_vec()));
        }
        FLOAT => {
            return Result::Ok(Value::from_float(val.len() as f64));
        }
        INTEGER => {
            return Result::Ok(Value::from_int(val.len() as i64));
        }
        BOOL => {
            if val.len() == 0 {
                return Result::Ok(Value::from_bool(false));
            } else {
                return Result::Ok(Value::from_bool(true));
            }
        }
        STRING | TEXTBUFFER => {
            let mut out: String = "lambda[".to_string();
            for v in val {
                match v.conv(STRING) {
                    Ok(s_v) => {
                        out = out + &" ".to_string();
                        out = out + &s_v.cast_string().unwrap();
                        out = out + &" :: ".to_string();
                    }
                    Err(_) => continue,
                }
            }
            out = out + &"]".to_string();
            if t == STRING {
                return Result::Ok(Value::from_string(out));
            } else {
                return Result::Ok(Value::text_buffer(out));
            }
        }
        _ => Err(format!("Can not convert lambda to {:?}", &t).into()),
    }
}

fn true_value_map_conversion(
    t: u16,
    ot: u16,
    val: &HashMap<Value, Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != MAP && ot != ASSOCIATION && ot != INFO && ot != CONFIG && ot != MESSAGE && ot != CONDITIONAL {
        return Err(format!(
            "Source value is not MAP but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        VALUEMAP => {
            let mut res: HashMap<Value, Value> = HashMap::new();
            for (k, v) in val {
                res.insert(k.clone(), v.clone());
            }
            return Result::Ok(Value::from_valuemap(res));
        }
        FLOAT => {
            return Result::Ok(Value::from_float(val.len() as f64));
        }
        INTEGER => {
            return Result::Ok(Value::from_int(val.len() as i64));
        }
        BOOL => {
            if val.len() == 0 {
                return Result::Ok(Value::from_bool(false));
            } else {
                return Result::Ok(Value::from_bool(true));
            }
        }
        LIST => {
            let mut res: Vec<Value> = Vec::new();
            for (k, v) in val {
                res.push(Value::pair(k.clone(), v.clone()));
            }
            return Result::Ok(Value::from_list(res));
        }
        STRING | TEXTBUFFER => {
            let mut out: String = "{".to_string();
            for (k, v) in val {
                let key_str_value = match k.conv(STRING) {
                    Ok(key_str_value) => key_str_value,
                    Err(_err) =>  {
                        return Err(format!("Can not convert key in valuemap to {:?}", &k).into());
                    }
                };
                let key_str = match key_str_value.cast_string() {
                    Ok(key_str) => key_str,
                    Err(_err) =>  {
                        return Err(format!("Can not cast key in valuemap to {:?}", &k).into());
                    }
                };
                match v.conv(STRING) {
                    Ok(s_v) => {
                        out = out + &" ".to_string();
                        out = out + &key_str;
                        out = out + &"=".to_string();
                        out = out + &s_v.cast_string().unwrap();
                        out = out + &" :: ".to_string();
                    }
                    Err(_) => continue,
                }
            }
            out = out + &"}".to_string();
            if t == STRING {
                return Result::Ok(Value::from_string(out));
            } else {
                return Result::Ok(Value::text_buffer(out));
            }
        }
        _ => Err(format!("Can not convert valuemap to {:?}", &t).into()),
    }
}

fn value_map_conversion(
    t: u16,
    ot: u16,
    val: &HashMap<String, Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != MAP && ot != ASSOCIATION && ot != INFO && ot != CONFIG && ot != MESSAGE && ot != CONDITIONAL {
        return Err(format!(
            "Source value is not MAP but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }
    match t {
        MAP => {
            let mut res: HashMap<String, Value> = HashMap::new();
            for (k, v) in val {
                res.insert(k.clone(), v.clone());
            }
            return Result::Ok(Value::from_dict(res));
        }
        CONDITIONAL => {
            let mut res: HashMap<String, Value> = HashMap::new();
            for (k, v) in val {
                res.insert(k.clone(), v.clone());
            }
            return Result::Ok(Value::conditional_from_dict(res));
        }
        FLOAT => {
            return Result::Ok(Value::from_float(val.len() as f64));
        }
        INTEGER => {
            return Result::Ok(Value::from_int(val.len() as i64));
        }
        BOOL => {
            if val.len() == 0 {
                return Result::Ok(Value::from_bool(false));
            } else {
                return Result::Ok(Value::from_bool(true));
            }
        }
        LIST => {
            let mut res: Vec<Value> = Vec::new();
            for (k, v) in val {
                res.push(Value::pair(Value::from_string(k.clone()), v.clone()));
            }
            return Result::Ok(Value::from_list(res));
        }
        STRING | TEXTBUFFER => {
            let mut out: String = "{".to_string();
            for (k, v) in val {
                match v.conv(STRING) {
                    Ok(s_v) => {
                        out = out + &" ".to_string();
                        out = out + &k.to_string();
                        out = out + &"=".to_string();
                        out = out + &s_v.cast_string().unwrap();
                        out = out + &" :: ".to_string();
                    }
                    Err(_) => continue,
                }
            }
            out = out + &"}".to_string();
            if t == STRING {
                return Result::Ok(Value::from_string(out));
            } else {
                return Result::Ok(Value::text_buffer(out));
            }
        }
        _ => Err(format!("Can not convert map to {:?}", &t).into()),
    }
}

fn value_json_conversion(
    t: u16,
    ot: u16,
    val: &serde_json::Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    if ot != JSON {
        return Err(format!(
            "Source value is not JSON but {:?} and not suitable for conversion",
            &ot
        )
        .into());
    }

    match t {
        STRING | TEXTBUFFER => {
            let Ok(str_json) = serde_json::to_string(val) else { return Err(format!("Error casting JSON to string").into()); };
            return Ok(Value::from_string(str_json));
        }
        _ => {
            match Value::json(val.clone()).cast_json_to_value() {
                Ok(value) => {
                    return value.conv(ot);
                }
                Err(err) => {
                    return Err(format!("Can not convert json to INTEGER: {:?} due to: {}", &t, err).into());
                }
            }
        }
    }
}

impl Value {
    pub fn conv(&self, t: u16) -> Result<Self, Box<dyn std::error::Error>> {
        match &self.data {
            Val::F64(f_val) => value_float_conversion(t, self.dt, *f_val),
            Val::I64(i_val) => value_integer_conversion(t, self.dt, *i_val),
            Val::String(s_val) => match self.type_of() {
                    STRING => value_string_conversion(t, self.dt, &s_val),
                    CALL => value_call_conversion(t, self.dt, s_val.to_string()),
                    PTR => value_ptr_conversion(t, self.dt, s_val.to_string()),
                    _ => value_string_conversion(t, self.dt, &s_val),
            },
            Val::Bool(b_val) => value_bool_conversion(t, self.dt, *b_val),
            Val::Json(j_val) => value_json_conversion(t, self.dt, &j_val),
            _ => match self.dt {
                LIST | RESULT => match &self.data {
                    Val::List(l_val) => value_list_conversion(t, self.dt, l_val),
                    _ => {
                        Err(format!("Can not convert LIST/RESULT Value from {:?}", &self.dt).into())
                    }
                },
                MATRIX => match &self.data {
                    Val::Matrix(m_val) => value_matrix_conversion(t, self.dt, m_val),
                    _ => {
                        Err(format!("Can not convert MATRIX Value from {:?}", &self.dt).into())
                    }
                },
                QUEUE | FIFO => match &self.data {
                    Val::Queue(q_val) => value_queue_conversion(t, self.dt, q_val),
                    _ => {
                        Err(format!("Can not convert QUEUE/FIFO Value from {:?}", &self.dt).into())
                    }
                },
                LAMBDA => match &self.data {
                    Val::Lambda(l_val) => value_lambda_conversion(t, self.dt, l_val),
                    _ => Err(format!("Can not convert LAMBDA Value from {:?}", &self.dt).into()),
                },
                MAP | INFO | CONFIG | ASSOCIATION | MESSAGE | CONDITIONAL | CLASS | OBJECT => match &self.data {
                    Val::Map(m_val) => value_map_conversion(t, self.dt, m_val),
                    _ => Err(format!("Can not convert MAP Value from {:?}", &self.dt).into()),
                },
                VALUEMAP => match &self.data {
                    Val::ValueMap(m_val) => true_value_map_conversion(t, self.dt, m_val),
                    _ => Err(format!("Can not convert VALUEMAP Value from {:?}", &self.dt).into()),
                },
                BIN => match &self.data {
                    Val::Binary(b_val) => value_bin_conversion(t, self.dt, b_val.to_vec()),
                    _ => Err(format!("Can not convert BIN Value from {:?}", &self.dt).into()),
                },
                NODATA => value_nodata_conversion(t, self.dt),
                NONE => value_none_conversion(t, self.dt),
                _ => Err(format!("Can not convert Value from {:?}", &self.dt).into()),
            },
        }
    }
}
