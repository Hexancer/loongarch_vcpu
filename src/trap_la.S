.section .text

.global _hyp_trap_vector
_hyp_trap_vector:
    // 保存 host 寄存器状态
    csrwr   $r3, 0x502    // 保存 r3 到 DESAVE CSR
    csrrd   $r3, 0x33     // 从 SAVE3 CSR 获取上下文指针
    
    // 保存 host 通用寄存器
    st.d    $r0, $r3, 0
    st.d    $r1, $r3, 8
    st.d    $r2, $r3, 16
    st.d    $r4, $r3, 32
    st.d    $r5, $r3, 40
    st.d    $r6, $r3, 48
    st.d    $r7, $r3, 56
    st.d    $r8, $r3, 64
    st.d    $r9, $r3, 72
    st.d    $r10, $r3, 80
    st.d    $r11, $r3, 88
    st.d    $r12, $r3, 96
    st.d    $r13, $r3, 104
    st.d    $r14, $r3, 112
    st.d    $r15, $r3, 120
    st.d    $r16, $r3, 128
    st.d    $r17, $r3, 136
    st.d    $r18, $r3, 144
    st.d    $r19, $r3, 152
    st.d    $r20, $r3, 160
    st.d    $r21, $r3, 168
    st.d    $r22, $r3, 176
    st.d    $r23, $r3, 184
    st.d    $r24, $r3, 192
    st.d    $r25, $r3, 200
    st.d    $r26, $r3, 208
    st.d    $r27, $r3, 216
    st.d    $r28, $r3, 224
    st.d    $r29, $r3, 232
    st.d    $r30, $r3, 240
    st.d    $r31, $r3, 248

    // 保存 host CSR 寄存器
    csrrd   $r4, 0x6      // ERA
    st.d    $r4, $r3, 256

    // 保存 guest 通用寄存器
    gcsrrd  $r4, 0x0      // GCSR_CRMD
    st.d    $r4, $r3, 256+8*1
    gcsrrd  $r4, 0x1      // GCSR_PRMD
    st.d    $r4, $r3, 256+8*2
    gcsrrd  $r4, 0x2      // GCSR_EUEN
    st.d    $r4, $r3, 256+8*3
    gcsrrd  $r4, 0x3      // GCSR_MISC
    st.d    $r4, $r3, 256+8*4
    gcsrrd  $r4, 0x4      // GCSR_ECTL
    st.d    $r4, $r3, 256+8*5
    gcsrrd  $r4, 0x5      // GCSR_ESTAT
    st.d    $r4, $r3, 256+8*6
    gcsrrd  $r4, 0x6      // GCSR_ERA
    st.d    $r4, $r3, 256+8*7
    gcsrrd  $r4, 0x7      // GCSR_BADV
    st.d    $r4, $r3, 256+8*8
    gcsrrd  $r4, 0x8      // GCSR_BADI
    st.d    $r4, $r3, 256+8*9

    // 调用 Rust 处理函数
    move    $r4, $r3      // 传递上下文指针
    bl      trap_handler

    // 恢复 guest CSR 寄存器
    ld.d    $r4, $r3, 256+8*1
    gcsrwr  $r4, 0x0      // GCSR_CRMD
    ld.d    $r4, $r3, 256+8*2
    gcsrwr  $r4, 0x1      // GCSR_PRMD
    ld.d    $r4, $r3, 256+8*3
    gcsrwr  $r4, 0x2      // GCSR_EUEN
    ld.d    $r4, $r3, 256+8*4
    gcsrwr  $r4, 0x3      // GCSR_MISC
    ld.d    $r4, $r3, 256+8*5
    gcsrwr  $r4, 0x4      // GCSR_ECTL
    ld.d    $r4, $r3, 256+8*6
    gcsrwr  $r4, 0x5      // GCSR_ESTAT
    ld.d    $r4, $r3, 256+8*7
    gcsrwr  $r4, 0x6      // GCSR_ERA
    ld.d    $r4, $r3, 256+8*8
    gcsrwr  $r4, 0x7      // GCSR_BADV
    ld.d    $r4, $r3, 256+8*9
    gcsrwr  $r4, 0x8      // GCSR_BADI

    // 恢复 host CSR 寄存器
    ld.d    $r4, $r3, 256 // ERA
    csrwr   $r4, 0x6

    // 恢复 host 通用寄存器
    ld.d    $r0, $r3, 0
    ld.d    $r1, $r3, 8
    ld.d    $r2, $r3, 16
    ld.d    $r4, $r3, 32
    ld.d    $r5, $r3, 40
    ld.d    $r6, $r3, 48
    ld.d    $r7, $r3, 56
    ld.d    $r8, $r3, 64
    ld.d    $r9, $r3, 72
    ld.d    $r10, $r3, 80
    ld.d    $r11, $r3, 88
    ld.d    $r12, $r3, 96
    ld.d    $r13, $r3, 104
    ld.d    $r14, $r3, 112
    ld.d    $r15, $r3, 120
    ld.d    $r16, $r3, 128
    ld.d    $r17, $r3, 136
    ld.d    $r18, $r3, 144
    ld.d    $r19, $r3, 152
    ld.d    $r20, $r3, 160
    ld.d    $r21, $r3, 168
    ld.d    $r22, $r3, 176
    ld.d    $r23, $r3, 184
    ld.d    $r24, $r3, 192
    ld.d    $r25, $r3, 200
    ld.d    $r26, $r3, 208
    ld.d    $r27, $r3, 216
    ld.d    $r28, $r3, 224
    ld.d    $r29, $r3, 232
    ld.d    $r30, $r3, 240
    ld.d    $r31, $r3, 248

    // 恢复 r3
    csrrd   $r3, 0x502
    ertn
