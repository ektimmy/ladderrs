 enum SimpleOperators {
    LD,
    LDN,
    ST,
    STN,
    NOT,
    S,
    R,
    CLK,
    CU,
    CD,
    PV,
    IN,
    PT,
    ExpressionOperators,
}

enum ExpressionOperators {
    AND, //Also includes the & string
    OR,
    XOR,
    ANDN, //Also includes the &N string
    ORN,
    XORN,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    GT,
    GE,
    EQ,
    LT,
    LE,
    NE,
}

enum CallOperators {
    CALL,
    CALLC,
    CALLCN,
}

enum ReturnOperator {
    RET,
    RETC,
    RETCN,
}

enum JumpOperators {
    JMP,
    JMPC,
    JMPCN,
}

