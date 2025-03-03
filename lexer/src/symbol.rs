use std::collections::HashSet;

pub struct Symbol {

}

pub struct SymbolTableEntry {
    symbol_type: String,
    scope: String,
    address: String,
    value: String,
    rwx: String,
}

pub struct SymbolTable {
    set: HashSet<SymbolTableEntry>
}