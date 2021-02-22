#[allow(dead_code)]
pub enum Extension {
    A = 0,  // Atomic extension
    B = 1,  // Tentatively reserved for Bit-Manipulation extension
    C = 2,  // Compressed extension
    D = 3,  // Double-precision floating-point extension
    E = 4,  // RV32E base ISA
    F = 5,  // Single-precision floating-point extension
    G = 6,  // Additional standard extensions present
    H = 7,  // Hypervisor extension
    I = 8,  // RV32I/64I/128I base ISA
    J = 9,  // Tentatively reserved for Dynamically Translated Languages extension
    K = 10, // Reserved
    L = 11, // Tentatively reserved for Decimal Floating-Point extension
    M = 12, // Integer Multiply/Divide extension
    N = 13, // User-level interrupts supported
    O = 14, // Reserved
    P = 15, // Tentatively reserved for Packed-SIMD extension
    Q = 16, // Quad-precision floating-point extension
    R = 17, // Reserved
    S = 18, // Supervisor mode implemented
    T = 19, // Tentatively reserved for Transactional Memory extension
    U = 20, // User mode implemented
    V = 21, // Tentatively reserved for Vector extension
    W = 22, // Reserved
    X = 23, // Non-standard extensions present
    Y = 24, // Reserved
    Z = 25, // Reserved
}
