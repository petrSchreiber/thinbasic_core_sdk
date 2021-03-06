/*

    Binding to thinCore - thinBasic internal engine.
    Functionality present here is used to interact with thinBasic.

*/

pub mod core
{
    extern crate libloading;    // For thinCore
    extern crate winapi;
    extern crate ascii;

    // Adjusting the signatures for our use
    extern "system" {
        pub fn SysAllocStringByteLen(
            psz: *const u8,
            len: usize,
            ) -> *const u8;

        pub fn SysFreeString(
            bstrString: *const u8
        );

        pub fn SysStringByteLen(
            bstr: *const u8
        ) -> u32;    
    }

    // Custom TBStr
    pub struct TBStr(*const u8);

    // For creating from Rust str
    impl<'a> From<&'a str> for TBStr {
        
        fn from(str_text: &'a str) -> Self {
            unsafe
            {
                let ascii_str = ascii::AsciiStr::from_ascii(str_text).unwrap();
                let byte_slice = ascii_str.as_bytes();
                let ptr = SysAllocStringByteLen(&byte_slice[0], ascii_str.len());

                TBStr(ptr)
            }
        }
    }

    // For releasing
    impl Drop for TBStr {
        #[inline(always)]
        fn drop(&mut self) {
            unsafe
            {
                SysFreeString(self.0);
            }
        }
    }

    // Convenient, custom functions
    impl TBStr
    {
        pub fn len(&self) -> u32 {
            unsafe {
                SysStringByteLen(self.0)
            }
        }
        
        pub fn to_string<'v>(&self) -> String {
            unsafe {
                let len = self.len();            
                let slice: &[u8] = ::std::slice::from_raw_parts(self.0, len as usize);

                String::from(::std::str::from_utf8(slice).unwrap())
            }
        }      
    }

    #[allow(dead_code)]
    pub enum ReturnType
    {
        NONE  =  0,
        U8    =  1,
        U16   =  3,
        U32   =  4,       
        I16   =  2,
        I32   =  5,
        I64   =  6,
        F32   =  7,
        F64   =  8,
        TBSTR = 30
    }

    /*

     Library setup

    */

    #[allow(dead_code)]
    pub fn add_function<T>(symbol_name: &str, function_ptr: extern fn() -> T, return_type: ReturnType) -> i32
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();

            let thinbasic_loadsymbol: libloading::Symbol<unsafe extern fn(symbol_name: TBStr, return_type: i32, function_ptr: extern fn() -> T, force_overwrite: i32) -> i32> = lib.get(b"thinBasic_LoadSymbol").unwrap();

            thinbasic_loadsymbol(TBStr::from(symbol_name), return_type as i32, function_ptr, 1)
        }
    }

    #[allow(dead_code)]
    pub fn add_string_equate(symbol_name: &str, symbol_value: &str) -> i32
    {
        unsafe
        {
            assert!(symbol_name.starts_with("$"));
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();

            // Placeholder values added as Rust does not support f80
            let thinbasic_addequate: libloading::Symbol<unsafe extern fn(symbol_name: TBStr, symbol_value: TBStr, placeholderA: i64, placeholderA: i16, equate_type: i32) -> i32> = lib.get(b"thinBasic_AddEquate").unwrap();

            // 0, 0 are placeholder values; 5 should enforce string type of equate
            thinbasic_addequate(TBStr::from(symbol_name), TBStr::from(symbol_value), 0, 0, 5)
        }
    }

    /*

     Parsing

    */

    #[allow(dead_code)]
    pub fn parse_i16() -> i16
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_parseinteger: libloading::Symbol<unsafe extern fn(*const i16)> = lib.get(b"thinBasic_ParseInteger").unwrap();
            let num: i16 = 0;
            thinbasic_parseinteger(&num);

            num
        }
    }

    #[allow(dead_code)]
    pub fn parse_i32() -> i32
    {
        unsafe         
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_parselong: libloading::Symbol<unsafe extern fn(*const i32)> = lib.get(b"thinBasic_ParseLong").unwrap();
            let num: i32 = 0;
            thinbasic_parselong(&num);

            num
        }
    }

    #[allow(dead_code)]
    pub fn parse_i64() -> i64
    {
        unsafe         
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_parsequad: libloading::Symbol<unsafe extern fn(*const i64)> = lib.get(b"thinBasic_ParseQuad").unwrap();
            let num: i64 = 0;
            thinbasic_parsequad(&num);

            num
        }
    }

    #[allow(dead_code)]
    pub fn parse_u8() -> u8
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_parsebyte: libloading::Symbol<unsafe extern fn(*const u8)> = lib.get(b"thinBasic_ParseByte").unwrap();
            let num: u8 = 0;
            thinbasic_parsebyte(&num);

            num
        }
    }

    #[allow(dead_code)]
    pub fn parse_u16() -> u16
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_parseword: libloading::Symbol<unsafe extern fn(*const u16)> = lib.get(b"thinBasic_ParseWord").unwrap();
            let num: u16 = 0;
            thinbasic_parseword(&num);

            num
        }
    }

    #[allow(dead_code)]
    pub fn parse_u32() -> u32
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_parsedword: libloading::Symbol<unsafe extern fn(*const u32)> = lib.get(b"thinBasic_ParseDWord").unwrap();
            let num: u32 = 0;
            thinbasic_parsedword(&num);

            num
        }
    }

    #[allow(dead_code)]
    pub fn parse_f32() -> f32
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_parsesingle: libloading::Symbol<unsafe extern fn(*const f32)> = lib.get(b"thinBasic_ParseSingle").unwrap();
            let num: f32 = 0.0;
            thinbasic_parsesingle(&num);

            num
        }
    }

    #[allow(dead_code)]
    pub fn parse_f64() -> f64
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_parsedouble: libloading::Symbol<unsafe extern fn(*const f64)> = lib.get(b"thinBasic_ParseDouble").unwrap();
            let num: f64 = 0.0;
            thinbasic_parsedouble(&num);

            num
        }
    }

    #[allow(dead_code)]
    pub fn parse_tbstr() -> TBStr
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_parsestr: libloading::Symbol<unsafe extern fn(*const TBStr)> = lib.get(b"thinBasic_ParseStr").unwrap();
            let text: TBStr = TBStr::from(" ");
            thinbasic_parsestr(&text);

            text
        }
    }

    #[allow(dead_code)]
    pub fn check_comma() -> bool
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_checkcomma_mandatory: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_CheckComma_Mandatory").unwrap();
            let result = thinbasic_checkcomma_mandatory();

            return if result == 0 { false } else { true }
        }
    }

    #[allow(dead_code)]
    pub fn check_comma_optional() -> bool
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_checkcomma_optional: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_CheckComma_Optional").unwrap();
            let result = thinbasic_checkcomma_optional();

            return if result == 0 { false } else { true }
        }
    }

    #[allow(dead_code)]
    pub fn check_open_parens() -> bool
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_checkopenparens_mandatory: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_CheckOpenParens_Mandatory").unwrap();
            let result = thinbasic_checkopenparens_mandatory();

            return if result == 0 { false } else { true }
        }
    }

    #[allow(dead_code)]
    pub fn check_open_parens_optional() -> bool
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_checkopenparens_optional: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_CheckOpenParens_Optional").unwrap();
            let result = thinbasic_checkopenparens_optional();

            return if result == 0 { false } else { true }
        }
    }

    #[allow(dead_code)]
    pub fn check_close_parens() -> bool
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_checkcloseparens_mandatory: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_CheckCloseParens_Mandatory").unwrap();
            let result = thinbasic_checkcloseparens_mandatory();

            return if result == 0 { false } else { true }
        }
    }

    #[allow(dead_code)]
    pub fn check_close_parens_optional() -> bool
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_checkcloseparens_optional: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_CheckCloseParens_Optional").unwrap();
            let result = thinbasic_checkcloseparens_optional();

            return if result == 0 { false } else { true }
        }
    }   


    /*

     Error handling

    */

    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum RunTimeError
    {
        NoError                                 = 0,
        Parens                                  = 1,
        NoExp                                   = 2,
        DivZero                                 = 3,
        EqualExp                                = 4,
        NotVar                                  = 5,
        LabTabFull                              = 6,
        DupLab                                  = 7,
        UndefTab                                = 8,
        ThenExpected                            = 9,
        ToExpected                              = 10,
        TooManyFor                              = 11,
        NextWithoutFor                          = 12,
        MissingSemicolon                        = 13,
        CommandRetNoneInExpression              = 14,

        MissQuote                               = 15,
        BadFile                                 = 16,
        StrExpected                             = 17,
        UnknownKeyword                          = 18,
        MissingOpenparens                       = 19,
        MissingCloseparens                      = 20,
        MissingComma                            = 21,
        MissingSquarecloseparens                = 22,

        EolIncorrect                            = 23,

        DoubleConcatenation                     = 24,
        CommandUnknownReturnedParameter         = 25,
        PrimitiveStr                            = 26,
        PrimitiveNum                            = 27,
        PrintError                              = 28,
        PrintErrorNoendofline                   = 29,
        VariableNotDefined                      = 30,
        AtomTokenTypeNumeric                    = 31,
        IfWithoutEndif                          = 32,
        IfEndifWithoutIf                        = 33,
        TooManyWhile                            = 34,
        WhileWithoutWend                        = 35,
        DimTypeNotDefined                       = 36,
        ExitNoKeyFound                          = 37,
        NoEndFunctionFound                      = 38,
        FunctionNameDuplicate                   = 39,
        FunctionNameIsKey                       = 40,
        FunctionNameIsVar                       = 41,
        FunctionNameIsUdt                       = 42,
        EndNoKeyFound                           = 43,

        DimMissingAs                            = 44,
        UndefinedToken                          = 45,
        UnsupportedChar                         = 46,

        FunctionDeclareMissingAs                = 47,
        DeclareFunctionOrSubExpected            = 48,

        DoLoopWrongCondition                    = 49,

        MissingAlias                            = 50,
        MissingLib                              = 51,
        UndefinedVarType                        = 52,
        ParameterByrefIncorrectPtr              = 53,

        LoopExpectedWhileOrUntil                = 55,

        EndFunctionWithoutFunction              = 58,
        EndFunctionFound                        = 59,

        IterateNoKeyFound                       = 60,

        WithEndWithError                        = 64,

        TooNestedDoloop                         = 65,
        DoLoopMissingDoOrLoop                   = 66,

        VariableNotDimensioned                  = 70,
        VariableIsNotArray                      = 71,
        VariableMustbeStringType                = 72,

        RedimNewTypeNotSupported                = 73,
        RedimPreserveNotValidAbsolute           = 74,

        VariableMustBeUdtType                   = 75,

        KeywordNotExpected                      = 76,
        TokenNotExpected                        = 77,

        IncludeFileNotFound                     = 80,

        DimUnexpectedKeyword                    = 85,

        FunctionNotSupported                    = 90,

        ArrayFunctionNotSupported               = 91,

        UdtElementNotFound                      = 100,
        UdtExpected                             = 101,
        UdtEquOrElementExpected                 = 102,

        AssignmentNotSupported                  = 110,

        RelationalExpected                      = 115,

        ApiLibNotFound                          = 120,
        ApiFunctionNotFoundInLib                = 121,
        ApiGeneralAddressNotPresent             = 122,

        CallNotSupportedStatement               = 130,
        CallFunctionNotFound                    = 131,

        FunctionNotFound                        = 133,
        FunctionExpectedCallback                = 134,

        EquateAlreadyDefined                    = 135,
        EquateAlreadyDefinedDifferent           = 136,

        VariableNameDuplicateGlobal             = 142,
        VariableNameDuplicateFunction           = 144,
        VariableNameDuplicateLocal              = 145,
        VariableNameDuplicate                   = 146,

        ForStepShouldBeNegative                 = 150,
        ForStepShouldBePositive                 = 151,
        ForExpectedAVariable                    = 152,
        ForStepIsZero                           = 153,
        ForVarMustBeNumeric                     = 154,

        AliasCommandNameExpected                = 160,
        AliasAsExpected                         = 161,
        AliasUndefNotUndef                      = 162,

        TypeMissingEndUnion                     = 169,
        TypeMissingEndType                      = 170,
        TypeMissingEndClass                     = 171,
        TypeTypeNotDefined                      = 172,
        TypeMissingAs                           = 173,
        TypeNameMustbeUndefined                 = 174,
        TypeArrayMustbeDimensioned              = 175,
        TypeStringsMustHaveSize                 = 176,
        TypeElementAlreadyPresent               = 177,
        TypeElementAlreadyPresentInherit        = 178,
        TypeDynstringInsideUnion                = 179,

        NoEndRawtextFound                       = 180,

        BeginBlockUnsuported                    = 190,
        BeginConstMissingEnd                    = 191,

        FunctionParamUnrecognizedType           = 220,

        DoWithoutLoop                           = 245,
        FunctionMissingAs                       = 246,
        RegexprMissingTo                        = 247,
        RegexprMissingIn                        = 248,
        SelectWithoutEndSelect                  = 249,
        DuplicateSymbol                         = 250,
        InvalidnumericChar                      = 251,
        InvalidDelimiter                        = 252,
        InvalidDataType                         = 253,
        VariableExpected                        = 254,
        VariableVariantExpected                 = 255,

        SelectMissingCase                       = 270,
        SelectErrorKindofOperation              = 271,
        SelectCodeBetweenSelectCase             = 272,

        StrptrVariableNotADynstringNum          = 280,
        StrptrVariableNotADynstringVar          = 281,
        StrptrVariableNotADynstringUdt          = 282,

        ApicallRefExpected                      = 300,

        ArrayOutOfBound                         = 400,

        ModuleSpecific                          = 500,

        PreparserDirectiveNotSupported          = 800,

        PreparserScriptVersionRequest           = 820,

        InternalReturnMainType                  = 900,

        InternalDecription                      = 910,

        InternalUdtBufferShort                  = 915,

        InternalReturnNoneNoCodePtr             = 921,
        InternalReturnNumberNoCodePtr           = 922,
        InternalReturnStringNoCodePtr           = 923,

        ClassNewNoIndexAllowed                  = 5010,
        ClassNewDifferentClass                  = 5015,
        ClassNewNoClass                         = 5020,
        ClassNewExpectedNew                     = 5025,
        ClassNotInitWithNew                     = 5030,
        ClassSetNowAllowed                      = 5035,

        ClassMethodPropertyNotfound             = 5100,
        ClassExpected                           = 5110,

        TraceStopByUser                         = 11000,

        ObfuscationFileNotValid                 = 12000,

        ComGeneric                              = 30000
    }

    #[allow(dead_code)]
    pub fn get_last_error() -> RunTimeError
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_getlasterror: libloading::Symbol<unsafe extern fn() -> RunTimeError> = lib.get(b"thinBasic_GetLastError").unwrap();
            let result = thinbasic_getlasterror();

            return result;
        }
    }

    #[allow(dead_code)]
    pub fn error_free() -> bool
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_errorfree: libloading::Symbol<unsafe extern fn() -> i32> = lib.get(b"thinBasic_ErrorFree").unwrap();
            let result = thinbasic_errorfree();

            return if result == 0 { false } else { true }
        }
    }

    #[allow(dead_code)]
    pub fn raise_runtime_error(error_type: RunTimeError, description: &str) -> bool
    {
        unsafe
        {
            let lib: libloading::Library = libloading::Library::new("thinCore.dll").unwrap();
            let thinbasic_runtimeerror: libloading::Symbol<unsafe extern fn(error_type: i32, error_description: TBStr) -> i32> = lib.get(b"thinBasic_RunTimeError").unwrap();

            let result = thinbasic_runtimeerror(error_type as i32, TBStr::from(description));

            return if result == 0 { false } else { true }
        }
    }
}
