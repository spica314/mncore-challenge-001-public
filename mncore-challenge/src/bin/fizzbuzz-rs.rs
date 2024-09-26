// 方針の検証用コード

fn main() {
    let mut s = String::new();

    for i in 1..=6334 {
        if i % 15 == 0 {
            s.push_str("FizzBuzz\n");
        } else if i % 3 == 0 {
            s.push_str("Fizz\n");
        } else if i % 5 == 0 {
            s.push_str("Buzz\n");
        } else {
            s.push_str(&i.to_string());
            s.push_str("\n");
        }
        if i == 9 {
            eprintln!("{}", s.len());
        }
        if i == 99 {
            eprintln!("{}", s.len());
        }
        if i == 999 {
            eprintln!("{}", s.len());
        }
    }
    s.pop();

    eprintln!("{}", s.len());
    eprintln!("{}", s.len() / 8);

    let ss: Vec<_> = s.chars().collect();


    let mut res = String::new();
    for i in 0..4096 {
        let mut r = String::new();

        let start = i * 8;
        let end = start + 8;

        /* 直前長語から考えて、fizzbuzzの対象の値, 何byte目から書き込み */
        let mut local_value = 0;
        let mut local_offset = 0i64;

        let mut nd = 0;
        let mut base = 999;
        let mut start2 = 0;
        let mut cycle = 80;
        let mut a = 0;
        let mut start3 = 0;
        if start >= 4668 {
            nd = 5;
            base = 999;
            start2 = start - 4668;
            cycle = 79;
            a = start2 / cycle;
            start3 = start2 - a * cycle;
        } else if start >= 408 {
            nd = 4;
            base = 99;
            start2 = start - 408;
            cycle = 71;
            a = start2 / cycle;
            start3 = start2 - a * cycle;
        } else if start >= 30 {
            nd = 3;
            base = 9;
            start2 = start - 30;
            cycle = 63;
            a = start2 / cycle;
            start3 = start2 - a * cycle;
        }


        //  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14
        //  8  3  3  4  3  4  4  3  3  4  4  3  4  3  3
        //  9  4  4  5  4  5  5  4  4  5  5  4  5  4  4
        // 100以降のbyte数
        // let start2 = start - 408;
        // 1周期分 = 1 * 9 + 6 * 5 + 8 * 4 = 71
        // 周期数
        // let a = start2 / cycle;
        // 周期のあまり
        // let start3 = start2 - a * cycle;
        local_value = base + 15 * a;
        let mut cu = 0;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += 5;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += nd;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += 5;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += nd;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += nd;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += 9;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += nd;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += nd;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += 5;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += nd;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += 5;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += 5;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += nd;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += nd;

        if start3 >= cu {
            let start4 = start3 - cu;
            local_offset = 16 - start4;
            local_value += 1;
        }
        cu += 5;



        let mut local_offset = local_offset as usize;

        let mut digits = vec![0; 4];
        digits[3] = local_value % 10;
        digits[2] = local_value / 10 % 10;
        digits[1] = local_value / 100 % 10;
        digits[0] = local_value / 1000 % 10;

        let mut buf = [b'_'; 1024];
        for _ in 0..4 {
            if local_value % 15 == 0 {
                buf[local_offset] = b'F';
                buf[local_offset + 1] = b'i';
                buf[local_offset + 2] = b'z';
                buf[local_offset + 3] = b'z';
                buf[local_offset + 4] = b'B';
                buf[local_offset + 5] = b'u';
                buf[local_offset + 6] = b'z';
                buf[local_offset + 7] = b'z';
                buf[local_offset + 8] = b'\n';
                local_offset += 9;
            } else if local_value % 3 == 0 {
                buf[local_offset] = b'F';
                buf[local_offset + 1] = b'i';
                buf[local_offset + 2] = b'z';
                buf[local_offset + 3] = b'z';
                buf[local_offset + 4] = b'\n';
                local_offset += 5;
            } else if local_value % 5 == 0 {
                buf[local_offset] = b'B';
                buf[local_offset + 1] = b'u';
                buf[local_offset + 2] = b'z';
                buf[local_offset + 3] = b'z';
                buf[local_offset + 4] = b'\n';
                local_offset += 5;
            } else {
                if digits[0] > 0 {
                    buf[local_offset] = b'0' + digits[0] as u8;
                    local_offset += 1;
                    buf[local_offset] = b'0' + digits[1] as u8;
                    local_offset += 1;
                    buf[local_offset] = b'0' + digits[2] as u8;
                    local_offset += 1;
                    buf[local_offset] = b'0' + digits[3] as u8;
                    local_offset += 1;
                } else if digits[1] > 0 {
                    buf[local_offset] = b'0' + digits[1] as u8;
                    local_offset += 1;
                    buf[local_offset] = b'0' + digits[2] as u8;
                    local_offset += 1;
                    buf[local_offset] = b'0' + digits[3] as u8;
                    local_offset += 1;
                } else if digits[2] > 0 {
                    buf[local_offset] = b'0' + digits[2] as u8;
                    local_offset += 1;
                    buf[local_offset] = b'0' + digits[3] as u8;
                    local_offset += 1;
                } else if digits[3] > 0 {
                    buf[local_offset] = b'0' + digits[3] as u8;
                    local_offset += 1;
                }
                buf[local_offset] = b'\n';
                local_offset += 1;
            }
            local_value += 1;
            digits[3] += 1;
            if digits[3] >= 10 {
                digits[3] = 0;
                digits[2] += 1;
            }
            if digits[2] >= 10 {
                digits[2] = 0;
                digits[1] += 1;
            }
            if digits[1] >= 10 {
                digits[1] = 0;
                digits[0] += 1;
            }
        }
        r = std::str::from_utf8(&buf[16..24]).unwrap().to_string();


        if i == 3001 {
            eprintln!("i = {}", i);
            eprintln!("local_value = {}", local_value);
            eprintln!("local_offset = {}", local_offset);
            eprintln!("start,end = {}, {}", start, end);
            eprintln!("expected = {:?}", &ss[start as usize..end as usize].iter().collect::<String>());
            eprintln!("actual   = {:?}", r);
        }

        assert!(r.len() == 8);
        res.push_str(&r);
    }
    let res_s: Vec<_> = res.chars().collect();

    let mut ok = 0;
    let mut ng = 0;
    for i in 0..32766 {
        if ss[i] == res_s[i] {
            ok += 1;
        } else {
            ng += 1;
            if ng < 100000 {
                eprintln!("i = {}, expected = {}, actual = {}", i, ss[i], res_s[i]);
            }
        }
    }
    eprintln!("ok = {}, ng = {}", ok, ng);

    for i in 0..4668 {
        eprintln!("i = {}, expected = {:?}, actual = {:?}", i, ss[i], res_s[i]);
    }
}
