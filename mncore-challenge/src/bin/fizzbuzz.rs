// FizzBuzzのVSMを生成するコード

// x * yをtargetに格納
fn gen_multi(x: &str, y: i64, target: &str, op_mask: &str) {
    assert!(target != "$nowrite");
    assert!(x != target);
    println!("zero {target}/{op_mask}");
    for i in 0..32 {
        if y & (1 << i) != 0 {
            println!("ilsl {x} $r{} $nowrite", 300+i);
            println!("iadd {target} $aluf {target}/{op_mask}");
        }
    }
}

// xがmax_x未満のときにx / yをtargetに格納
// 値によってはたぶんバグる
fn gen_div(x: &str, y: i64, target: &str, max_x: i64, op_mask: &str) {
    let max_x_next_2 = ((max_x as usize).next_power_of_two() as i64) << 6;
    let max_x_next_2_log2 = max_x_next_2.trailing_zeros();
    let mul = (max_x_next_2 + y - 1) / y;
    eprintln!("y = {}, mul = {}, max = {}", y, mul, max_x * mul);
    for i in 0..max_x {
        assert_eq!(i / y, (i * mul) >> max_x_next_2_log2, "i = {}", i);
    }
    gen_multi(x, mul, target, op_mask);
    println!("ilsr $aluf $r{} {target}/{op_mask}", 300+max_x_next_2_log2);
}

// (x & mask) << shiftをtargetに足す
fn iand_lshift_add(x: &str, mask: i64, shift: i64, target: &str, op_mask: &str) {
    println!("iand {x} $r{} $nowrite", 300+mask);
    println!("ilsl $aluf $r{} $nowrite", 300+shift);
    println!("iadd {target} $aluf {target}/{op_mask}");
}

// x << shiftをtargetに足す
fn ilshift_add(x: &str, shift: i64, target: &str, op_mask: &str) {
    println!("ilsl {x} $r{} $nowrite", 300+shift);
    println!("iadd {target} $aluf {target}/{op_mask}");
}

// xがmax_x未満のときにx % yをtargetに格納
fn mod_y(x: &str, y: i64, target: &str, max_x: i64, tmp: &str, op_mask: &str) {
    gen_div(x, y, target, max_x, op_mask);
    gen_multi(target, y, tmp, op_mask);
    println!("isub {x} $aluf {target}/{op_mask}");
}

// x >= yのときにtargetのマスクレジスタに1をセット
fn cmp_geq(x: &str, y: i64, target: &str, op_mask: &str) {
    println!("imm i\"{y}\" $nowrite");
    println!("isub {x} $aluf {target}/{op_mask}");
}

// x >= yのときにtargetのマスクレジスタに1をセット
fn cmp_geq_rr(x: &str, y: &str, target: &str, op_mask: &str) {
    println!("isub {x} {y} {target}/{op_mask}");
}


fn main() {
    // $r{300+i}を固定値でiを入れておく
    for i in 0..=48 {
        println!("imm i\"{}\" $r{} $s{}", i, 300+i, 300+i);
    }

    // p_of_4096 = ($peid & 0b111) | (($mabid & 0b1110) << 5) | ($l1bid << 3) | ($l2bid << 9)
    let p_of_4096 = "$s263";
    println!("zero {p_of_4096}/1000");
    iand_lshift_add("$peid", 0b111, 0, p_of_4096, "1000");
    iand_lshift_add("$mabid", 0b1110, 5, p_of_4096, "1000");
    iand_lshift_add("$l1bid", 0b111, 3, p_of_4096, "1000");
    iand_lshift_add("$l2bid", 0b111, 9, p_of_4096, "1000");
    // println!("d getf {p_of_4096} 1");

    // let start = p_of_4096 * 8;
    let start = "$s264";
    println!("ilsl $aluf $r303 {start}/1000");


    // let end = start + 8;
    let end = "$s265";
    println!("iadd $aluf $r308 {end}/1000");

    let local_value = "$s266";
    let local_offset = "$s267";
    let nd = "$s268";
    let base = "$s269";
    let start2 = "$s270";
    let cycle = "$s271";
    let a = "$s272";
    let start3 = "$s273";
    let tmp0 = "$r274";

    // if start >= 30 {
    //     nd = 3;
    //     base = 9;
    //     start2 = start - 30;
    //     cycle = 63;
    //     a = start2 / cycle;
    //     tmp0 = a * cycle;
    //     start3 = start2 - tmp0;
    // }
    cmp_geq(start, 30, "$omr1", "1000");
    println!("imm i\"3\" {nd}/$imr1");
    println!("imm i\"9\" {base}/$imr1");
    println!("imm i\"30\" $nowrite");
    println!("isub {start} $aluf {start2}/$imr1");
    println!("imm i\"63\" {cycle}/$imr1");
    gen_div(start2, 63, a, 32768, "$imr1");
    gen_multi(a, 63, tmp0, "$imr1");
    println!("isub {start2} $aluf {start3}/$imr1");

    // start >= 408 {
    //     nd = 4;
    //     base = 99;
    //     start2 = start - 408;
    //     cycle = 71;
    //     a = start2 / cycle;
    //     tmp0 = a * cycle;
    //     start3 = start2 - tmp0;
    // }
    cmp_geq(start, 408, "$omr1", "1000");
    println!("imm i\"4\" {nd}/$imr1");
    println!("imm i\"99\" {base}/$imr1");
    println!("imm i\"408\" $nowrite");
    println!("isub {start} $aluf {start2}/$imr1");
    println!("imm i\"71\" {cycle}/$imr1");
    gen_div(start2, 71, a, 32768, "$imr1");
    gen_multi(a, 71, tmp0, "$imr1");
    println!("isub {start2} $aluf {start3}/$imr1");

    // if start >= 4668 {
    //     nd = 5;
    //     base = 999;
    //     start2 = start - 4668;
    //     cycle = 79;
    //     a = start2 / cycle;
    //     tmp0 = a * cycle;
    //     start3 = start2 - tmp0;
    // }
    cmp_geq(start, 4668, "$omr1", "1000");
    println!("imm i\"5\" {nd}/$imr1");
    println!("imm i\"999\" {base}/$imr1");
    println!("imm i\"4668\" $nowrite");
    println!("isub {start} $aluf {start2}/$imr1");
    println!("imm i\"79\" {cycle}/$imr1");
    gen_div(start2, 79, a, 32768, "$imr1");
    gen_multi(a, 79, tmp0, "$imr1");
    println!("isub {start2} $aluf {start3}/$imr1");

    // local_value = base + 15 * a;
    gen_multi(a, 15, local_value, "1000");
    println!("iadd {base} $aluf {local_value}/1000");

    println!("d getf {base} 1");

    // cu = 0;
    let cu = "$r275";
    // println!("zero {cu}/1000");
    // println!("nop");

    // start4
    let start4 = "$s276";

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += 5;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} $s305 {cu}/1000");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += nd;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} {nd} {cu}/$imr1");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += 5;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} $s305 {cu}/$imr1");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += nd;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} {nd} {cu}/$imr1");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += nd;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} {nd} {cu}/$imr1");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += 9;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} $s309 {cu}/$imr1");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += nd;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} {nd} {cu}/$imr1");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += nd;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} {nd} {cu}/$imr1");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += 5;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} $s305 {cu}/$imr1");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += nd;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} {nd} {cu}/$imr1");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += 5;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} $s305 {cu}/$imr1");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += 5;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} $s305 {cu}/$imr1");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += nd;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} {nd} {cu}/$imr1");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    // cu += nd;
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} {nd} {cu}/$imr1");
    println!("nop");

    // if start3 >= cu {
    //     let start4 = start3 - cu;
    //     local_offset = 16 - start4;
    //     local_value += 1;
    // }
    cmp_geq_rr(start3, cu, "$omr1", "1000");
    println!("isub {start3} {cu} {start4}/$imr1");
    println!("isub $r316 $aluf {local_offset}/$imr1");
    println!("iadd {local_value} $r301 {local_value}/$imr1");
    println!("iadd {cu} $s305 {cu}/$imr1");
    println!("nop");

    // println!("d getf {local_value} 1");

    // digits = 0;
    let digits0 = "$s277";
    let digits1 = "$s278";
    let digits2 = "$s279";
    let digits3 = "$s280";
    let tmp1 = "$s281";
    let tmp2 = "$r282";
    let tmp3 = "$r283";
    let tmp4 = "$r284";
    let tmp5 = "$r285";

    // tmp1 = local_value / 10
    // tmp2 = tmp1 * 10
    // digits3 = local_value - tmp2
    gen_div(local_value, 10, tmp1, 8192, "1000");
    gen_multi(tmp1, 10, tmp2, "1000");
    println!("isub {local_value} $aluf {digits3}/1000");
    // tmp4 = tmp1
    // tmp2 = tmp1 / 10
    // tmp1 = tmp2
    // tmp2 = tmp1 * 10
    // digits2 = tmp4 - tmp2
    println!("ipassa {tmp1} {tmp4}/1000");
    gen_div(tmp1, 10, tmp2, 8192, "1000");
    println!("ipassa $aluf {tmp1}/1000");
    gen_multi(tmp1, 10, tmp2, "1000");
    println!("isub {tmp4} $aluf {digits2}/1000");
    // tmp4 = tmp1
    // tmp2 = tmp1 / 10
    // tmp1 = tmp2
    // tmp2 = tmp1 * 10
    // digits1 = tmp4 - tmp2
    println!("ipassa {tmp1} {tmp4}/1000");
    gen_div(tmp1, 10, tmp2, 8192, "1000");
    println!("ipassa $aluf {tmp1}/1000");
    gen_multi(tmp1, 10, tmp2, "1000");
    println!("isub {tmp4} $aluf {digits1}/1000");
    // tmp4 = tmp1
    // tmp2 = tmp1 / 10
    // tmp1 = tmp2
    // tmp2 = tmp1 * 10
    // digits0 = tmp4 - tmp2
    println!("ipassa {tmp1} {tmp4}/1000");
    gen_div(tmp1, 10, tmp2, 8192, "1000");
    println!("ipassa $aluf {tmp1}/1000");
    gen_multi(tmp1, 10, tmp2, "1000");
    println!("isub {tmp4} $aluf {digits0}/1000");

    // println!("d getf {local_offset} 1");

    for _ in 0..4 {
        println!("imm i\"0\" {tmp3}");

        // tmp0 = local_value % 15
        mod_y(local_value, 15, tmp1, 8192, tmp0, "1000");
        // if tmp0 == 0 {
        //     FizzBuzz\n
        // }
        println!("ior $aluf $aluf $omr1");
        println!("ipassa {local_offset} $r401/$imr1");
        println!("nop\nnop");
        println!("lpassa $lr400 $lt/$imr1");
        println!("imm i\"{}\" $nowrite", b'F');
        println!("ipassa $aluf $mt0/$imr1");
        println!("imm i\"{}\" $nowrite", b'i');
        println!("ipassa $aluf $mt1/$imr1");
        println!("imm i\"{}\" $nowrite", b'z');
        println!("ipassa $aluf $mt2/$imr1");
        println!("imm i\"{}\" $nowrite", b'z');
        println!("ipassa $aluf $mt3/$imr1");
        println!("imm i\"{}\" $nowrite", b'B');
        println!("ipassa $aluf $mt4/$imr1");
        println!("imm i\"{}\" $nowrite", b'u');
        println!("ipassa $aluf $mt5/$imr1");
        println!("imm i\"{}\" $nowrite", b'z');
        println!("ipassa $aluf $mt6/$imr1");
        println!("imm i\"{}\" $nowrite", b'z');
        println!("ipassa $aluf $mt7/$imr1");
        println!("imm i\"{}\" $nowrite", b'\n');
        println!("ipassa $aluf $mt8/$imr1");
        println!("iadd {local_offset} $r309 {local_offset}/$imr1");
        println!("imm i\"1\" {tmp3}/$imr1");

        // tmp0 = local_value % 3
        mod_y(local_value, 3, tmp1, 8192, tmp0, "1000");
        println!("iadd $aluf {tmp3} {tmp0}/1000");
        // if tmp0 == 0 {
        //     Fizz\n
        // }
        println!("ior $aluf $aluf $omr1");
        println!("ipassa {local_offset} $r401/$imr1");
        println!("nop\nnop");
        println!("lpassa $lr400 $lt/$imr1");
        println!("imm i\"{}\" $nowrite", b'F');
        println!("ipassa $aluf $mt0/$imr1");
        println!("imm i\"{}\" $nowrite", b'i');
        println!("ipassa $aluf $mt1/$imr1");
        println!("imm i\"{}\" $nowrite", b'z');
        println!("ipassa $aluf $mt2/$imr1");
        println!("imm i\"{}\" $nowrite", b'z');
        println!("ipassa $aluf $mt3/$imr1");
        println!("imm i\"{}\" $nowrite", b'\n');
        println!("ipassa $aluf $mt4/$imr1");
        println!("iadd {local_offset} $r305 {local_offset}/$imr1");
        println!("imm i\"1\" {tmp3}/$imr1");

        // tmp0 = local_value % 5
        mod_y(local_value, 5, tmp1, 8192, tmp0, "1000");
        println!("iadd $aluf {tmp3} {tmp0}/1000");
        // if tmp0 == 0 {
        //     Buzz\n
        // }
        println!("ior $aluf $aluf $omr1");
        println!("ipassa {local_offset} $r401/$imr1");
        println!("nop\nnop");
        println!("lpassa $lr400 $lt/$imr1");
        println!("imm i\"{}\" $nowrite", b'B');
        println!("ipassa $aluf $mt0/$imr1");
        println!("imm i\"{}\" $nowrite", b'u');
        println!("ipassa $aluf $mt1/$imr1");
        println!("imm i\"{}\" $nowrite", b'z');
        println!("ipassa $aluf $mt2/$imr1");
        println!("imm i\"{}\" $nowrite", b'z');
        println!("ipassa $aluf $mt3/$imr1");
        println!("imm i\"{}\" $nowrite", b'\n');
        println!("ipassa $aluf $mt4/$imr1");
        println!("iadd {local_offset} $r305 {local_offset}/$imr1");
        println!("imm i\"1\" {tmp3}/$imr1");
        println!("nop");
        println!("nop");

        // else
        println!("ior {tmp3} {tmp3} $omr1");
        // println!("iadd {local_offset} $r305 {local_offset}/$imr1");
        // println!("nop\nnop");
        println!("ipassa {local_offset} $r401/$imr1");
        println!("nop\nnop");
        println!("lpassa $lr400 $lt/$imr1");
        println!("nop");
        println!("nop");
        // if digits[0] > 0
        println!("ipassa {tmp3} {tmp5}/1000");
        println!("isub $r300 {digits0} $omr2/1000");
        println!("imm i\"1\" $nowrite");
        println!("ipassa $aluf {tmp5}/$imr2");
        println!("nop");
        println!("ior {tmp5} {tmp5} $omr2/1000");
        println!("iadd $r348 {digits0} $mt0/$imr2");
        println!("iadd $r348 {digits1} $mt1/$imr2");
        println!("iadd $r348 {digits2} $mt2/$imr2");
        println!("iadd $r348 {digits3} $mt3/$imr2");
        println!("ipassa $r310 $mt4/$imr2");
        println!("iadd {local_offset} $r305 {local_offset}/$imr2");
        println!("imm i\"1\" {tmp3}/$imr2");
        println!("nop");
        // if digits[1] > 0
        println!("ipassa {tmp3} {tmp5}/1000");
        println!("isub $r300 {digits1} $omr2/1000");
        println!("imm i\"1\" $nowrite");
        println!("ipassa $aluf {tmp5}/$imr2");
        println!("nop");
        println!("ior {tmp5} {tmp5} $omr2/1000");
        println!("iadd $r348 {digits1} $mt0/$imr2");
        println!("iadd $r348 {digits2} $mt1/$imr2");
        println!("iadd $r348 {digits3} $mt2/$imr2");
        println!("ipassa $r310 $mt3/$imr2");
        println!("iadd {local_offset} $r304 {local_offset}/$imr2");
        println!("imm i\"1\" {tmp3}/$imr2");
        println!("nop");
        // if digits[2] > 0
        println!("ipassa {tmp3} {tmp5}/1000");
        println!("isub $r300 {digits2} $omr2/1000");
        println!("imm i\"1\" $nowrite");
        println!("ipassa $aluf {tmp5}/$imr2");
        println!("nop");
        println!("ior {tmp5} {tmp5} $omr2/1000");
        println!("iadd $r348 {digits2} $mt0/$imr2");
        println!("iadd $r348 {digits3} $mt1/$imr2");
        println!("ipassa $r310 $mt2/$imr2");
        println!("iadd {local_offset} $r303 {local_offset}/$imr2");
        println!("imm i\"1\" {tmp3}/$imr2");






        // local_value += 1
        println!("iadd {local_value} $r301 {local_value}/1000");
        // digits[3] += 1
        println!("iadd {digits3} $r301 {digits3}/1000");
        // if digits[3] >= 10 {
        //     digits[2] += 1;
        //     digits[3] = 0;
        // }
        cmp_geq(digits3, 10, "$omr1", "1000");
        println!("iadd {digits2} $r301 {digits2}/$imr1");
        println!("ipassa $r300 {digits3}/$imr1");
        // if digits[2] >= 10 {
        //     digits[1] += 1;
        //     digits[2] = 0;
        // }
        cmp_geq(digits2, 10, "$omr1", "1000");
        println!("iadd {digits1} $r301 {digits1}/$imr1");
        println!("ipassa $r300 {digits2}/$imr1");
        // if digits[1] >= 10 {
        //     digits[0] += 1;
        //     digits[1] = 0;
        // }
        cmp_geq(digits1, 10, "$omr1", "1000");
        println!("iadd {digits0} $r301 {digits0}/$imr1");
        println!("ipassa $r300 {digits1}/$imr1");
    }

    // embedding - 0
    println!("isub $r300 {p_of_4096} {tmp1}/1000");
    println!("ior $aluf $aluf $omr1");
    println!("imm i\"{}\" $nowrite", b'1');
    println!("ipassa $aluf $m16/$imr1");
    println!("imm i\"{}\" $nowrite", b'\n');
    println!("ipassa $aluf $m17/$imr1");
    println!("imm i\"{}\" $nowrite", b'2');
    println!("ipassa $aluf $m18/$imr1");
    println!("imm i\"{}\" $nowrite", b'\n');
    println!("ipassa $aluf $m19/$imr1");
    println!("imm i\"{}\" $nowrite", b'F');
    println!("ipassa $aluf $m20/$imr1");
    println!("imm i\"{}\" $nowrite", b'i');
    println!("ipassa $aluf $m21/$imr1");
    println!("imm i\"{}\" $nowrite", b'z');
    println!("ipassa $aluf $m22/$imr1");
    println!("imm i\"{}\" $nowrite", b'z');
    println!("ipassa $aluf $m23/$imr1");

    // embedding - 1
    println!("isub $r301 {p_of_4096} {tmp1}/1000");
    println!("ior $aluf $aluf $omr1");
    println!("imm i\"{}\" $nowrite", b'\n');
    println!("ipassa $aluf $m16/$imr1");
    println!("imm i\"{}\" $nowrite", b'4');
    println!("ipassa $aluf $m17/$imr1");
    println!("imm i\"{}\" $nowrite", b'\n');
    println!("ipassa $aluf $m18/$imr1");
    println!("imm i\"{}\" $nowrite", b'B');
    println!("ipassa $aluf $m19/$imr1");
    println!("imm i\"{}\" $nowrite", b'u');
    println!("ipassa $aluf $m20/$imr1");
    println!("imm i\"{}\" $nowrite", b'z');
    println!("ipassa $aluf $m21/$imr1");
    println!("imm i\"{}\" $nowrite", b'z');
    println!("ipassa $aluf $m22/$imr1");
    println!("imm i\"{}\" $nowrite", b'\n');
    println!("ipassa $aluf $m23/$imr1");

    // embedding - 2
    println!("isub $r302 {p_of_4096} {tmp1}/1000");
    println!("ior $aluf $aluf $omr1");
    println!("imm i\"{}\" $nowrite", b'F');
    println!("ipassa $aluf $m16/$imr1");
    println!("imm i\"{}\" $nowrite", b'i');
    println!("ipassa $aluf $m17/$imr1");
    println!("imm i\"{}\" $nowrite", b'z');
    println!("ipassa $aluf $m18/$imr1");
    println!("imm i\"{}\" $nowrite", b'z');
    println!("ipassa $aluf $m19/$imr1");
    println!("imm i\"{}\" $nowrite", b'\n');
    println!("ipassa $aluf $m20/$imr1");
    println!("imm i\"{}\" $nowrite", b'7');
    println!("ipassa $aluf $m21/$imr1");
    println!("imm i\"{}\" $nowrite", b'\n');
    println!("ipassa $aluf $m22/$imr1");
    println!("imm i\"{}\" $nowrite", b'8');
    println!("ipassa $aluf $m23/$imr1");

    // embedding - 3
    println!("isub $r303 {p_of_4096} {tmp1}/1000");
    println!("ior $aluf $aluf $omr1");
    println!("imm i\"{}\" $nowrite", b'\n');
    println!("ipassa $aluf $m16/$imr1");
    println!("imm i\"{}\" $nowrite", b'F');
    println!("ipassa $aluf $m17/$imr1");
    println!("imm i\"{}\" $nowrite", b'i');
    println!("ipassa $aluf $m18/$imr1");
    println!("imm i\"{}\" $nowrite", b'z');
    println!("ipassa $aluf $m19/$imr1");
    println!("imm i\"{}\" $nowrite", b'z');
    println!("ipassa $aluf $m20/$imr1");
    println!("imm i\"{}\" $nowrite", b'\n');
    println!("ipassa $aluf $m21/$imr1");
    println!("imm i\"{}\" $nowrite", b'B');
    println!("ipassa $aluf $m22/$imr1");
    println!("imm i\"{}\" $nowrite", b'u');
    println!("ipassa $aluf $m23/$imr1");


    println!("nop");
    println!("nop");

    ilshift_add("$m16", 24, "$r0", "1000");
    ilshift_add("$m17", 16, "$r0", "1000");
    ilshift_add("$m18", 8, "$r0", "1000");
    ilshift_add("$m19", 0, "$r0", "1000");
    ilshift_add("$m20", 24, "$r1", "1000");
    ilshift_add("$m21", 16, "$r1", "1000");
    ilshift_add("$m22", 8, "$r1", "1000");
    ilshift_add("$m23", 0, "$r1", "1000");

    // println!("d getd $lr0 1");

    // DRAMまでもっていく
    println!(r#"nop
nop
l1bmd $lr0 $lb0
nop
nop
l2bmd $lb0 $lc0
l2bmd $lb32 $lc256
nop
nop
mvp/n512 $lc0@0.0 $d0@0
mvp/n512 $lc0@0.1 $d512@0
mvp/n512 $lc0@1.0 $d1024@0
mvp/n512 $lc0@1.1 $d1536@0
mvp/n512 $lc0@2.0 $d2048@0
mvp/n512 $lc0@2.1 $d2560@0
mvp/n512 $lc0@3.0 $d3072@0
mvp/n512 $lc0@3.1 $d3584@0
"#);
}
