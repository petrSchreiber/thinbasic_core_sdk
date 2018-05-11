/*

    Binding to thinCore - thinBasic internal engine.
    Functionality present here is used to interact with thinBasic.

*/

pub mod core
{
    extern crate libloading;    // For thinCore
    extern crate winapi;
    extern crate ascii;

    /*

        Implementation of "ThinBASIC STRING" - TBstr - is tortured code from
        BStr implementation made in https://github.com/contextfree/winrt-rust

    */

    use core::winapi::um::oleauto::{SysAllocStringByteLen, SysFreeString, SysStringLen};

    pub struct TBStr(*mut u16);

    impl<'a> From<&'a str> for TBStr {
        
        fn from(str_text: &'a str) -> Self {
            let max_ascii_length = str_text.chars().count();                            // Maximum length of ASCII string will not be bigger than original text
            let mut byte_buffer: Vec<u8> = Vec::with_capacity(max_ascii_length);        // Preallocate to avoid reallocations

            for str_char in str_text.chars() {                                          // For each wide char in str...         
                if str_char.is_ascii()                                                  // If it is representable as ASCII...
                {
                    let ascii_char = ascii::AsciiChar::from(str_char);                  // Convert to byte and push to byte_buffer
                    byte_buffer.push(ascii_char.unwrap().as_byte());
                }
            }

            let first_byte_of_byte_buffer = &byte_buffer[0] as *const _ as *const i8;   // Pointer to first item in our byte buffer, formed in a way SysAllocStringByteLen will love it
            let total_string_data_length =  byte_buffer.len() as u32;                   // Total length of the buffer, just casted in a way SysAllocStringByteLen will love it
            let tbstr = unsafe {
                SysAllocStringByteLen(first_byte_of_byte_buffer,
                                      total_string_data_length)
            };
            
            TBStr(tbstr)                                                                // Filling the TBStr with the value returned by SysAllocStringByteLen
        }
    }

    impl Drop for TBStr {
        #[inline(always)]
        fn drop(&mut self) {
            unsafe { SysFreeString(self.0 as *mut u16) };
        }
    }

    impl TBStr{
        #[allow(dead_code)]
        pub fn ptr(&self) -> &*mut u16 {
            return &self.0
        }

        #[allow(dead_code)]
        pub fn len(&self) -> u32 {        
            unsafe { SysStringLen(self.0 as *mut u16) }
        }

        #[allow(dead_code)]
        fn internal_to_string(&self) -> String {
            unsafe {
                let len = self.len();
                let slice: &[u16] = ::std::slice::from_raw_parts(self.0, len as usize);
                String::from_utf16_lossy(slice)
            }
        }
    }

    #[allow(dead_code)]
    pub enum ReturnType
    {
        None = 0,
        U8  =  1,
        U16 =  3,
        U32 =  4,       
        I16 =  2,
        I32 =  5,
        I64 =  6,
        F32 =  7,
        F64 =  8
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
