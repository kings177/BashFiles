use crate::runtime::{*};
use std::sync::atomic::{AtomicBool, Ordering};

// Precomps
// --------

pub struct PrecompFuns {
  pub visit: VisitFun,
  pub apply: ApplyFun,
}

pub struct Precomp {
  pub id: u64,
  pub name: &'static str,
  pub smap: &'static [bool],
  pub funs: Option<PrecompFuns>,
}

pub const STRING_NIL : u64 = 0;
pub const STRING_CONS : u64 = 1;
pub const BOTH : u64 = 2;
pub const KIND_TERM_CT0 : u64 = 3;
pub const KIND_TERM_CT1 : u64 = 4;
pub const KIND_TERM_CT2 : u64 = 5;
pub const KIND_TERM_CT3 : u64 = 6;
pub const KIND_TERM_CT4 : u64 = 7;
pub const KIND_TERM_CT5 : u64 = 8;
pub const KIND_TERM_CT6 : u64 = 9;
pub const KIND_TERM_CT7 : u64 = 10;
pub const KIND_TERM_CT8 : u64 = 11;
pub const KIND_TERM_CT9 : u64 = 12;
pub const KIND_TERM_CTA : u64 = 13;
pub const KIND_TERM_CTB : u64 = 14;
pub const KIND_TERM_CTC : u64 = 15;
pub const KIND_TERM_CTD : u64 = 16;
pub const KIND_TERM_CTE : u64 = 17;
pub const KIND_TERM_CTF : u64 = 18;
pub const KIND_TERM_CTG : u64 = 19;
pub const KIND_TERM_U60 : u64 = 20;
pub const KIND_TERM_F60 : u64 = 21;
pub const U60_IF : u64 = 22;
pub const U60_SWAP : u64 = 23;
pub const HVM_LOG : u64 = 24;
pub const HVM_QUERY : u64 = 25;
pub const HVM_PRINT : u64 = 26;
pub const HVM_SLEEP : u64 = 27;
pub const HVM_STORE : u64 = 28;
pub const HVM_LOAD : u64 = 29;
pub const _String_nil_ : u64 = 0;
pub const _String_cons_ : u64 = 1;
pub const _Both_ : u64 = 2;
pub const _Kind_Term_ct0_ : u64 = 3;
pub const _Kind_Term_ct1_ : u64 = 4;
pub const _Kind_Term_ct2_ : u64 = 5;
pub const _Kind_Term_ct3_ : u64 = 6;
pub const _Kind_Term_ct4_ : u64 = 7;
pub const _Kind_Term_ct5_ : u64 = 8;
pub const _Kind_Term_ct6_ : u64 = 9;
pub const _Kind_Term_ct7_ : u64 = 10;
pub const _Kind_Term_ct8_ : u64 = 11;
pub const _Kind_Term_ct9_ : u64 = 12;
pub const _Kind_Term_ctA_ : u64 = 13;
pub const _Kind_Term_ctB_ : u64 = 14;
pub const _Kind_Term_ctC_ : u64 = 15;
pub const _Kind_Term_ctD_ : u64 = 16;
pub const _Kind_Term_ctE_ : u64 = 17;
pub const _Kind_Term_ctF_ : u64 = 18;
pub const _Kind_Term_ctG_ : u64 = 19;
pub const _Kind_Term_u60_ : u64 = 20;
pub const _Kind_Term_f60_ : u64 = 21;
pub const _U60_if_ : u64 = 22;
pub const _U60_swap_ : u64 = 23;
pub const _HVM_log_ : u64 = 24;
pub const _HVM_query_ : u64 = 25;
pub const _HVM_print_ : u64 = 26;
pub const _HVM_sleep_ : u64 = 27;
pub const _HVM_store_ : u64 = 28;
pub const _HVM_load_ : u64 = 29;
pub const _Gen_go_ : u64 = 30;
pub const _Leaf_ : u64 = 31;
pub const _Node_ : u64 = 32;
pub const _Sum_ : u64 = 33;
pub const _Null_ : u64 = 34;
pub const _Sort_ : u64 = 35;
pub const _ToArr_ : u64 = 36;
pub const _ToMap_ : u64 = 37;
pub const _Free_ : u64 = 38;
pub const _Used_ : u64 = 39;
pub const _Main_ : u64 = 40;
pub const _Reverse_ : u64 = 41;
pub const _Gen_ : u64 = 42;
pub const _Merge_ : u64 = 43;
pub const _BOTH_ : u64 = 44;
pub const _Radix_ : u64 = 45;

pub const PRECOMP : &[Precomp] = &[
  Precomp {
    id: STRING_NIL,
    name: "String.nil",
    smap: &[false; 0],
    funs: None,
  },
  Precomp {
    id: STRING_CONS,
    name: "String.cons",
    smap: &[false; 2],
    funs: None,
  },
  Precomp {
    id: BOTH,
    name: "Both",
    smap: &[false; 2],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT0,
    name: "Kind.Term.ct0",
    smap: &[false; 2],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT1,
    name: "Kind.Term.ct1",
    smap: &[false; 3],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT2,
    name: "Kind.Term.ct2",
    smap: &[false; 4],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT3,
    name: "Kind.Term.ct3",
    smap: &[false; 5],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT4,
    name: "Kind.Term.ct4",
    smap: &[false; 6],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT5,
    name: "Kind.Term.ct5",
    smap: &[false; 7],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT6,
    name: "Kind.Term.ct6",
    smap: &[false; 8],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT7,
    name: "Kind.Term.ct7",
    smap: &[false; 9],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT8,
    name: "Kind.Term.ct8",
    smap: &[false; 10],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT9,
    name: "Kind.Term.ct9",
    smap: &[false; 11],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTA,
    name: "Kind.Term.ctA",
    smap: &[false; 12],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTB,
    name: "Kind.Term.ctB",
    smap: &[false; 13],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTC,
    name: "Kind.Term.ctC",
    smap: &[false; 14],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTD,
    name: "Kind.Term.ctD",
    smap: &[false; 15],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTE,
    name: "Kind.Term.ctE",
    smap: &[false; 16],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTF,
    name: "Kind.Term.ctF",
    smap: &[false; 17],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTG,
    name: "Kind.Term.ctG",
    smap: &[false; 18],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_U60,
    name: "Kind.Term.u60",
    smap: &[false; 2],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_F60,
    name: "Kind.Term.f60",
    smap: &[false; 2],
    funs: None,
  },
  Precomp {
    id: U60_IF,
    name: "U60.if",
    smap: &[true, false, false],
    funs: Some(PrecompFuns {
      visit: u60_if_visit,
      apply: u60_if_apply,
    }),
  },
  Precomp {
    id: U60_SWAP,
    name: "U60.swap",
    smap: &[true, false, false],
    funs: Some(PrecompFuns {
      visit: u60_swap_visit,
      apply: u60_swap_apply,
    }),
  },
  Precomp {
    id: HVM_LOG,
    name: "HVM.log",
    smap: &[false; 2],
    funs: Some(PrecompFuns {
      visit: hvm_log_visit,
      apply: hvm_log_apply,
    }),
  },
  Precomp {
    id: HVM_QUERY,
    name: "HVM.query",
    smap: &[false; 1],
    funs: Some(PrecompFuns {
      visit: hvm_query_visit,
      apply: hvm_query_apply,
    }),
  },
  Precomp {
    id: HVM_PRINT,
    name: "HVM.print",
    smap: &[false; 2],
    funs: Some(PrecompFuns {
      visit: hvm_print_visit,
      apply: hvm_print_apply,
    }),
  },
  Precomp {
    id: HVM_SLEEP,
    name: "HVM.sleep",
    smap: &[false; 2],
    funs: Some(PrecompFuns {
      visit: hvm_sleep_visit,
      apply: hvm_sleep_apply,
    }),
  },
  Precomp {
    id: HVM_STORE,
    name: "HVM.store",
    smap: &[false; 3],
    funs: Some(PrecompFuns {
      visit: hvm_store_visit,
      apply: hvm_store_apply,
    }),
  },
  Precomp {
    id: HVM_LOAD,
    name: "HVM.load",
    smap: &[false; 2],
    funs: Some(PrecompFuns {
      visit: hvm_load_visit,
      apply: hvm_load_apply,
    }),
  },
  Precomp {
    id: _Gen_go_,
    name: "Gen.go",
    smap: &[true, false],
    funs: Some(PrecompFuns {
      visit: _Gen_go__visit,
      apply: _Gen_go__apply,
    }),
  },
  Precomp {
    id: _Leaf_,
    name: "Leaf",
    smap: &[false],
    funs: None,
  },
  Precomp {
    id: _Node_,
    name: "Node",
    smap: &[false, false],
    funs: None,
  },
  Precomp {
    id: _Sum_,
    name: "Sum",
    smap: &[true],
    funs: Some(PrecompFuns {
      visit: _Sum__visit,
      apply: _Sum__apply,
    }),
  },
  Precomp {
    id: _Null_,
    name: "Null",
    smap: &[],
    funs: None,
  },
  Precomp {
    id: _Sort_,
    name: "Sort",
    smap: &[false],
    funs: Some(PrecompFuns {
      visit: _Sort__visit,
      apply: _Sort__apply,
    }),
  },
  Precomp {
    id: _ToArr_,
    name: "ToArr",
    smap: &[false, true],
    funs: Some(PrecompFuns {
      visit: _ToArr__visit,
      apply: _ToArr__apply,
    }),
  },
  Precomp {
    id: _ToMap_,
    name: "ToMap",
    smap: &[true],
    funs: Some(PrecompFuns {
      visit: _ToMap__visit,
      apply: _ToMap__apply,
    }),
  },
  Precomp {
    id: _Free_,
    name: "Free",
    smap: &[],
    funs: None,
  },
  Precomp {
    id: _Used_,
    name: "Used",
    smap: &[],
    funs: None,
  },
  Precomp {
    id: _Main_,
    name: "Main",
    smap: &[false],
    funs: Some(PrecompFuns {
      visit: _Main__visit,
      apply: _Main__apply,
    }),
  },
  Precomp {
    id: _Reverse_,
    name: "Reverse",
    smap: &[true],
    funs: Some(PrecompFuns {
      visit: _Reverse__visit,
      apply: _Reverse__apply,
    }),
  },
  Precomp {
    id: _Gen_,
    name: "Gen",
    smap: &[false],
    funs: Some(PrecompFuns {
      visit: _Gen__visit,
      apply: _Gen__apply,
    }),
  },
  Precomp {
    id: _Merge_,
    name: "Merge",
    smap: &[true, true],
    funs: Some(PrecompFuns {
      visit: _Merge__visit,
      apply: _Merge__apply,
    }),
  },
  Precomp {
    id: _BOTH_,
    name: "BOTH",
    smap: &[true, true],
    funs: Some(PrecompFuns {
      visit: _BOTH__visit,
      apply: _BOTH__apply,
    }),
  },
  Precomp {
    id: _Radix_,
    name: "Radix",
    smap: &[false],
    funs: Some(PrecompFuns {
      visit: _Radix__visit,
      apply: _Radix__apply,
    }),
  },
];

pub const PRECOMP_COUNT : u64 = PRECOMP.len() as u64;

// Ul0.if (cond: Term) (if_t: Term) (if_f: Term)
// ---------------------------------------------

#[inline(always)]
pub fn u60_if_visit(ctx: ReduceCtx) -> bool {
  if is_whnf(load_arg(ctx.heap, ctx.term, 0)) {
    return false;
  } else {
    let goup = ctx.redex.insert(ctx.tid, new_redex(*ctx.host, *ctx.cont, 1));
    *ctx.cont = goup;
    *ctx.host = get_loc(ctx.term, 0);
    return true;
  }
}

#[inline(always)]
pub fn u60_if_apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  let arg1 = load_arg(ctx.heap, ctx.term, 1);
  let arg2 = load_arg(ctx.heap, ctx.term, 2);
  if get_tag(arg0) == SUP {
    fun::superpose(ctx.heap, &ctx.prog.aris, ctx.tid, *ctx.host, ctx.term, arg0, 0);
  }
  if (get_tag(arg0) == U60) {
    if (get_num(arg0) == 0) {
      inc_cost(ctx.heap, ctx.tid);
      let done = arg2;
      link(ctx.heap, *ctx.host, done);
      collect(ctx.heap, &ctx.prog.aris, ctx.tid, arg1);
      free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 3);
      return true;
    } else {
      inc_cost(ctx.heap, ctx.tid);
      let done = arg1;
      link(ctx.heap, *ctx.host, done);
      collect(ctx.heap, &ctx.prog.aris, ctx.tid, arg2);
      free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 3);
      return true;
    }
  }
  return false;
}

// U60.swap (cond: Term) (pair: Term)
// ----------------------------------

#[inline(always)]
pub fn u60_swap_visit(ctx: ReduceCtx) -> bool {
  if is_whnf(load_arg(ctx.heap, ctx.term, 0)) {
    return false;
  } else {
    let goup = ctx.redex.insert(ctx.tid, new_redex(*ctx.host, *ctx.cont, 1));
    *ctx.cont = goup;
    *ctx.host = get_loc(ctx.term, 0);
    return true;
  }
}

#[inline(always)]
pub fn u60_swap_apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  let arg1 = load_arg(ctx.heap, ctx.term, 1);
  let arg2 = load_arg(ctx.heap, ctx.term, 2);
  if get_tag(arg0) == SUP {
    fun::superpose(ctx.heap, &ctx.prog.aris, ctx.tid, *ctx.host, ctx.term, arg0, 0);
  }
  if (get_tag(arg0) == U60) {
    if (get_num(arg0) == 0) {
      inc_cost(ctx.heap, ctx.tid);
      let ctr_0 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, ctr_0 + 0, arg1);
      link(ctx.heap, ctr_0 + 1, arg2);
      let done = Ctr(BOTH, ctr_0);
      link(ctx.heap, *ctx.host, done);
      free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 3);
      return true;
    } else {
      inc_cost(ctx.heap, ctx.tid);
      let ctr_0 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, ctr_0 + 0, arg2);
      link(ctx.heap, ctr_0 + 1, arg1);
      let done = Ctr(BOTH, ctr_0);
      link(ctx.heap, *ctx.host, done);
      free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 3);
      return true;
    }
  }
  return false;
}

// HVM.log (term: Term)
// --------------------

fn hvm_log_visit(ctx: ReduceCtx) -> bool {
  return false;
}

fn hvm_log_apply(ctx: ReduceCtx) -> bool {
  normalize(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 0), false);
  let code = crate::language::readback::as_code(ctx.heap, ctx.prog, get_loc(ctx.term, 0));
  println!("{}", code);
  link(ctx.heap, *ctx.host, load_arg(ctx.heap, ctx.term, 1));
  collect(ctx.heap, &ctx.prog.aris, ctx.tid, load_ptr(ctx.heap, get_loc(ctx.term, 0)));
  free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
  return true;
}

// HVM.query (cont: String -> Term)
// --------------------------------

fn hvm_query_visit(ctx: ReduceCtx) -> bool {
  return false;
}

fn hvm_query_apply(ctx: ReduceCtx) -> bool {
  fn read_input() -> String {
    use std::io::{stdin,stdout,Write};
    let mut input = String::new();
    stdin().read_line(&mut input).expect("string");
    if let Some('\n') = input.chars().next_back() { input.pop(); }
    if let Some('\r') = input.chars().next_back() { input.pop(); }
    return input;
  }
  let cont = load_arg(ctx.heap, ctx.term, 0);
  let text = make_string(ctx.heap, ctx.tid, &read_input());
  let app0 = alloc(ctx.heap, ctx.tid, 2);
  link(ctx.heap, app0 + 0, cont);
  link(ctx.heap, app0 + 1, text);
  free(ctx.heap, 0, get_loc(ctx.term, 0), 1);
  let done = App(app0);
  link(ctx.heap, *ctx.host, done);
  return true;
}

// HVM.print (text: String) (cont: Term)
// -----------------------------------------------

fn hvm_print_visit(ctx: ReduceCtx) -> bool {
  return false;
}

fn hvm_print_apply(ctx: ReduceCtx) -> bool {
  //normalize(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 0), false);
  if let Some(text) = crate::language::readback::as_string(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 0)) {
    println!("{}", text);
  }
  link(ctx.heap, *ctx.host, load_arg(ctx.heap, ctx.term, 1));
  collect(ctx.heap, &ctx.prog.aris, ctx.tid, load_ptr(ctx.heap, get_loc(ctx.term, 0)));
  free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
  return true;
}

// HVM.sleep (time: U60) (cont: Term)
// ----------------------------------

fn hvm_sleep_visit(ctx: ReduceCtx) -> bool {
  return false;
}

fn hvm_sleep_apply(ctx: ReduceCtx) -> bool {
  let time = reduce(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 0), true, false);
  std::thread::sleep(std::time::Duration::from_nanos(get_num(time)));
  link(ctx.heap, *ctx.host, load_ptr(ctx.heap, get_loc(ctx.term, 1)));
  free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
  return true;
}

// HVM.store (key: String) (val: String) (cont: Term)
// --------------------------------------------------

fn hvm_store_visit(ctx: ReduceCtx) -> bool {
  return false;
}

fn hvm_store_apply(ctx: ReduceCtx) -> bool {
  if let Some(key) = crate::language::readback::as_string(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 0)) {
    if let Some(val) = crate::language::readback::as_string(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 1)) {
      if std::fs::write(key, val).is_ok() {
        //let app0 = alloc(ctx.heap, ctx.tid, 2);
        //link(ctx.heap, app0 + 0, cont);
        //link(ctx.heap, app0 + 1, U6O(0));
        //free(ctx.heap, 0, get_loc(ctx.term, 0), 2);
        let done = load_arg(ctx.heap, ctx.term, 2);
        link(ctx.heap, *ctx.host, done);
        collect(ctx.heap, &ctx.prog.aris, ctx.tid, load_arg(ctx.heap, ctx.term, 0));
        collect(ctx.heap, &ctx.prog.aris, ctx.tid, load_arg(ctx.heap, ctx.term, 1));
        free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 3);
        return true;
      }
    }
  }
  println!("Runtime failure on: {}", show_at(ctx.heap, ctx.prog, *ctx.host, &[]));
  std::process::exit(0);
}

// HVM.load (key: String) (cont: String -> Term)
// ---------------------------------------------

fn hvm_load_visit(ctx: ReduceCtx) -> bool {
  return false;
}

fn hvm_load_apply(ctx: ReduceCtx) -> bool {
  if let Some(key) = crate::language::readback::as_string(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 0)) {
    if let Ok(file) = std::fs::read(key) {
      if let Ok(file) = std::str::from_utf8(&file) {
        let cont = load_arg(ctx.heap, ctx.term, 1); 
        let text = make_string(ctx.heap, ctx.tid, file);
        let app0 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, app0 + 0, cont);
        link(ctx.heap, app0 + 1, text);
        free(ctx.heap, 0, get_loc(ctx.term, 0), 2);
        let done = App(app0);
        link(ctx.heap, *ctx.host, done);
        return true;
      }
    }
  }
  println!("Runtime failure on: {}", show_at(ctx.heap, ctx.prog, *ctx.host, &[]));
  std::process::exit(0);
}

#[inline(always)]
pub fn _Gen_go__visit(ctx: ReduceCtx) -> bool {
  let mut vlen = 0;
  let vbuf = unsafe { ctx.heap.vbuf.get_unchecked(ctx.tid) };
  if !is_whnf(load_arg(ctx.heap, ctx.term, 0)) {
    unsafe { vbuf.get_unchecked(vlen) }.store(get_loc(ctx.term, 0), Ordering::Relaxed);
    vlen += 1
  }
  if vlen == 0 {
    return false;
  } else {
    let goup = ctx.redex.insert(ctx.tid, new_redex(*ctx.host, *ctx.cont, vlen as u64));
    if 0 < vlen - 1 {
      ctx.visit.push(new_visit(unsafe { vbuf.get_unchecked(0).load(Ordering::Relaxed) }, ctx.hold, goup));
    }
    *ctx.cont = goup;
    *ctx.host = unsafe { vbuf.get_unchecked(vlen - 1).load(Ordering::Relaxed) };
    return true;
  }
}

#[inline(always)]
pub fn _Gen_go__apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  let arg1 = load_arg(ctx.heap, ctx.term, 1);
  if get_tag(arg0) == SUP {
    fun::superpose(ctx.heap, &ctx.prog.aris, ctx.tid, *ctx.host, ctx.term, arg0, 0);
  }
  if (get_tag(arg0) == U60 && get_num(arg0) == 0) {
    inc_cost(ctx.heap, ctx.tid);
    let ctr_0 = alloc(ctx.heap, ctx.tid, 1);
    link(ctx.heap, ctr_0 + 0, arg1);
    let done = Ctr(_Leaf_, ctr_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
    return false;
  }
  if (get_tag(arg0) == CTR || get_tag(arg0) == U60 || get_tag(arg0) == F60) {
    inc_cost(ctx.heap, ctx.tid);
    let ret_3;
    if get_tag(arg1) == U60 && get_tag(U6O(1)) == U60 {
      ret_3 = U6O(u60::shl(get_num(arg1), get_num(U6O(1))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(arg1) == F60 && get_tag(U6O(1)) == F60 {
      ret_3 = F6O(f60::shl(get_num(arg1), get_num(U6O(1))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_4 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_4 + 0, arg1);
      link(ctx.heap, op2_4 + 1, U6O(1));
      ret_3 = Op2(SHL, op2_4);
    }
    let cpy_0 = ret_3;
    let dp0_1;
    let dp1_2;
    if get_tag(cpy_0) == U60 || get_tag(cpy_0) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_1 = cpy_0;
      dp1_2 = cpy_0;
    } else {
      let col_5 = gen_dup(ctx.heap, ctx.tid);
      let dup_6 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_6 + 0, Era());
    link(ctx.heap, dup_6 + 1, Era());
      link(ctx.heap, dup_6 + 2, cpy_0);
      dp0_1 = Dp0(col_5, dup_6);
      dp1_2 = Dp1(col_5, dup_6);
    }
    let ret_7;
    if get_tag(dp0_1) == U60 && get_tag(U6O(1)) == U60 {
      ret_7 = U6O(u60::or(get_num(dp0_1), get_num(U6O(1))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_1) == F60 && get_tag(U6O(1)) == F60 {
      ret_7 = F6O(f60::or(get_num(dp0_1), get_num(U6O(1))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_8 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_8 + 0, dp0_1);
      link(ctx.heap, op2_8 + 1, U6O(1));
      ret_7 = Op2(OR, op2_8);
    }
    let ret_12;
    if get_tag(arg0) == U60 && get_tag(U6O(1)) == U60 {
      ret_12 = U6O(u60::sub(get_num(arg0), get_num(U6O(1))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(arg0) == F60 && get_tag(U6O(1)) == F60 {
      ret_12 = F6O(f60::sub(get_num(arg0), get_num(U6O(1))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_13 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_13 + 0, arg0);
      link(ctx.heap, op2_13 + 1, U6O(1));
      ret_12 = Op2(SUB, op2_13);
    }
    let cpy_9 = ret_12;
    let dp0_10;
    let dp1_11;
    if get_tag(cpy_9) == U60 || get_tag(cpy_9) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_10 = cpy_9;
      dp1_11 = cpy_9;
    } else {
      let col_14 = gen_dup(ctx.heap, ctx.tid);
      let dup_15 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_15 + 0, Era());
    link(ctx.heap, dup_15 + 1, Era());
      link(ctx.heap, dup_15 + 2, cpy_9);
      dp0_10 = Dp0(col_14, dup_15);
      dp1_11 = Dp1(col_14, dup_15);
    }
    let cal_16 = get_loc(ctx.term, 0)/*reuse:2*/;
    link(ctx.heap, cal_16 + 0, dp0_10);
    link(ctx.heap, cal_16 + 1, dp1_2);
    let cal_17 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, cal_17 + 0, dp1_11);
    link(ctx.heap, cal_17 + 1, ret_7);
    let ctr_18 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, ctr_18 + 0, Fun(_Gen_go_, cal_16));
    link(ctx.heap, ctr_18 + 1, Fun(_Gen_go_, cal_17));
    let done = Ctr(_Node_, ctr_18);
    link(ctx.heap, *ctx.host, done);
    return false;
  }
  return false;
}

#[inline(always)]
pub fn _Sum__visit(ctx: ReduceCtx) -> bool {
  let mut vlen = 0;
  let vbuf = unsafe { ctx.heap.vbuf.get_unchecked(ctx.tid) };
  if !is_whnf(load_arg(ctx.heap, ctx.term, 0)) {
    unsafe { vbuf.get_unchecked(vlen) }.store(get_loc(ctx.term, 0), Ordering::Relaxed);
    vlen += 1
  }
  if vlen == 0 {
    return false;
  } else {
    let goup = ctx.redex.insert(ctx.tid, new_redex(*ctx.host, *ctx.cont, vlen as u64));
    if 0 < vlen - 1 {
      ctx.visit.push(new_visit(unsafe { vbuf.get_unchecked(0).load(Ordering::Relaxed) }, ctx.hold, goup));
    }
    *ctx.cont = goup;
    *ctx.host = unsafe { vbuf.get_unchecked(vlen - 1).load(Ordering::Relaxed) };
    return true;
  }
}

#[inline(always)]
pub fn _Sum__apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  if get_tag(arg0) == SUP {
    fun::superpose(ctx.heap, &ctx.prog.aris, ctx.tid, *ctx.host, ctx.term, arg0, 0);
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 34) {
    inc_cost(ctx.heap, ctx.tid);
    let done = U6O(0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(arg0, 0), 0);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 1);
    return false;
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 31) {
    inc_cost(ctx.heap, ctx.tid);
    let arg0_0 = load_arg(ctx.heap, arg0, 0);
    let done = arg0_0;
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(arg0, 0), 1);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 1);
    return true;
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 32) {
    inc_cost(ctx.heap, ctx.tid);
    let arg0_0 = load_arg(ctx.heap, arg0, 0);
    let arg0_1 = load_arg(ctx.heap, arg0, 1);
    let cal_2 = get_loc(ctx.term, 0)/*reuse:1*/;
    link(ctx.heap, cal_2 + 0, arg0_0);
    let cal_3 = alloc(ctx.heap, ctx.tid, 1);
    link(ctx.heap, cal_3 + 0, arg0_1);
    let ret_0;
    if get_tag(Fun(_Sum_, cal_2)) == U60 && get_tag(Fun(_Sum_, cal_3)) == U60 {
      ret_0 = U6O(u60::add(get_num(Fun(_Sum_, cal_2)), get_num(Fun(_Sum_, cal_3))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(Fun(_Sum_, cal_2)) == F60 && get_tag(Fun(_Sum_, cal_3)) == F60 {
      ret_0 = F6O(f60::add(get_num(Fun(_Sum_, cal_2)), get_num(Fun(_Sum_, cal_3))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_1 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_1 + 0, Fun(_Sum_, cal_2));
      link(ctx.heap, op2_1 + 1, Fun(_Sum_, cal_3));
      ret_0 = Op2(ADD, op2_1);
    }
    let done = ret_0;
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(arg0, 0), 2);
    return true;
  }
  return false;
}

#[inline(always)]
pub fn _Sort__visit(ctx: ReduceCtx) -> bool {
  return false;
}

#[inline(always)]
pub fn _Sort__apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  if true {
    inc_cost(ctx.heap, ctx.tid);
    let cal_0 = get_loc(ctx.term, 0)/*reuse:1*/;
    link(ctx.heap, cal_0 + 0, arg0);
    let cal_1 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, cal_1 + 0, U6O(0));
    link(ctx.heap, cal_1 + 1, Fun(_ToMap_, cal_0));
    let done = Fun(_ToArr_, cal_1);
    link(ctx.heap, *ctx.host, done);
    return true;
  }
  return false;
}

#[inline(always)]
pub fn _ToArr__visit(ctx: ReduceCtx) -> bool {
  let mut vlen = 0;
  let vbuf = unsafe { ctx.heap.vbuf.get_unchecked(ctx.tid) };
  if !is_whnf(load_arg(ctx.heap, ctx.term, 1)) {
    unsafe { vbuf.get_unchecked(vlen) }.store(get_loc(ctx.term, 1), Ordering::Relaxed);
    vlen += 1
  }
  if vlen == 0 {
    return false;
  } else {
    let goup = ctx.redex.insert(ctx.tid, new_redex(*ctx.host, *ctx.cont, vlen as u64));
    if 0 < vlen - 1 {
      ctx.visit.push(new_visit(unsafe { vbuf.get_unchecked(0).load(Ordering::Relaxed) }, ctx.hold, goup));
    }
    *ctx.cont = goup;
    *ctx.host = unsafe { vbuf.get_unchecked(vlen - 1).load(Ordering::Relaxed) };
    return true;
  }
}

#[inline(always)]
pub fn _ToArr__apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  let arg1 = load_arg(ctx.heap, ctx.term, 1);
  if get_tag(arg1) == SUP {
    fun::superpose(ctx.heap, &ctx.prog.aris, ctx.tid, *ctx.host, ctx.term, arg1, 1);
  }
  if (get_tag(arg1) == CTR && get_ext(arg1) == 38) {
    inc_cost(ctx.heap, ctx.tid);
    let ctr_0 = get_loc(arg1, 0)/*reuse:0*/;
    let done = Ctr(_Null_, ctr_0);
    link(ctx.heap, *ctx.host, done);
    collect(ctx.heap, &ctx.prog.aris, ctx.tid, arg0);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
    return false;
  }
  if (get_tag(arg1) == CTR && get_ext(arg1) == 39) {
    inc_cost(ctx.heap, ctx.tid);
    let ctr_0 = alloc(ctx.heap, ctx.tid, 1);
    link(ctx.heap, ctr_0 + 0, arg0);
    let done = Ctr(_Leaf_, ctr_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(arg1, 0), 0);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
    return false;
  }
  if (get_tag(arg1) == CTR && get_ext(arg1) == 2) {
    inc_cost(ctx.heap, ctx.tid);
    let arg1_0 = load_arg(ctx.heap, arg1, 0);
    let arg1_1 = load_arg(ctx.heap, arg1, 1);
    let cpy_0 = arg0;
    let dp0_1;
    let dp1_2;
    if get_tag(cpy_0) == U60 || get_tag(cpy_0) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_1 = cpy_0;
      dp1_2 = cpy_0;
    } else {
      let col_3 = gen_dup(ctx.heap, ctx.tid);
      let dup_4 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_4 + 0, Era());
    link(ctx.heap, dup_4 + 1, Era());
      link(ctx.heap, dup_4 + 2, cpy_0);
      dp0_1 = Dp0(col_3, dup_4);
      dp1_2 = Dp1(col_3, dup_4);
    }
    let ret_7;
    if get_tag(dp0_1) == U60 && get_tag(U6O(2)) == U60 {
      ret_7 = U6O(u60::mul(get_num(dp0_1), get_num(U6O(2))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_1) == F60 && get_tag(U6O(2)) == F60 {
      ret_7 = F6O(f60::mul(get_num(dp0_1), get_num(U6O(2))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_8 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_8 + 0, dp0_1);
      link(ctx.heap, op2_8 + 1, U6O(2));
      ret_7 = Op2(MUL, op2_8);
    }
    let ret_5;
    if get_tag(ret_7) == U60 && get_tag(U6O(0)) == U60 {
      ret_5 = U6O(u60::add(get_num(ret_7), get_num(U6O(0))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(ret_7) == F60 && get_tag(U6O(0)) == F60 {
      ret_5 = F6O(f60::add(get_num(ret_7), get_num(U6O(0))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_6 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_6 + 0, ret_7);
      link(ctx.heap, op2_6 + 1, U6O(0));
      ret_5 = Op2(ADD, op2_6);
    }
    let cal_9 = get_loc(arg1, 0)/*reuse:2*/;
    link(ctx.heap, cal_9 + 0, ret_5);
    link(ctx.heap, cal_9 + 1, arg1_0);
    let ret_12;
    if get_tag(dp1_2) == U60 && get_tag(U6O(2)) == U60 {
      ret_12 = U6O(u60::mul(get_num(dp1_2), get_num(U6O(2))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp1_2) == F60 && get_tag(U6O(2)) == F60 {
      ret_12 = F6O(f60::mul(get_num(dp1_2), get_num(U6O(2))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_13 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_13 + 0, dp1_2);
      link(ctx.heap, op2_13 + 1, U6O(2));
      ret_12 = Op2(MUL, op2_13);
    }
    let ret_10;
    if get_tag(ret_12) == U60 && get_tag(U6O(1)) == U60 {
      ret_10 = U6O(u60::add(get_num(ret_12), get_num(U6O(1))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(ret_12) == F60 && get_tag(U6O(1)) == F60 {
      ret_10 = F6O(f60::add(get_num(ret_12), get_num(U6O(1))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_11 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_11 + 0, ret_12);
      link(ctx.heap, op2_11 + 1, U6O(1));
      ret_10 = Op2(ADD, op2_11);
    }
    let cal_14 = get_loc(ctx.term, 0)/*reuse:2*/;
    link(ctx.heap, cal_14 + 0, ret_10);
    link(ctx.heap, cal_14 + 1, arg1_1);
    let ctr_15 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, ctr_15 + 0, Fun(_ToArr_, cal_9));
    link(ctx.heap, ctr_15 + 1, Fun(_ToArr_, cal_14));
    let done = Ctr(_Node_, ctr_15);
    link(ctx.heap, *ctx.host, done);
    return false;
  }
  return false;
}

#[inline(always)]
pub fn _ToMap__visit(ctx: ReduceCtx) -> bool {
  let mut vlen = 0;
  let vbuf = unsafe { ctx.heap.vbuf.get_unchecked(ctx.tid) };
  if !is_whnf(load_arg(ctx.heap, ctx.term, 0)) {
    unsafe { vbuf.get_unchecked(vlen) }.store(get_loc(ctx.term, 0), Ordering::Relaxed);
    vlen += 1
  }
  if vlen == 0 {
    return false;
  } else {
    let goup = ctx.redex.insert(ctx.tid, new_redex(*ctx.host, *ctx.cont, vlen as u64));
    if 0 < vlen - 1 {
      ctx.visit.push(new_visit(unsafe { vbuf.get_unchecked(0).load(Ordering::Relaxed) }, ctx.hold, goup));
    }
    *ctx.cont = goup;
    *ctx.host = unsafe { vbuf.get_unchecked(vlen - 1).load(Ordering::Relaxed) };
    return true;
  }
}

#[inline(always)]
pub fn _ToMap__apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  if get_tag(arg0) == SUP {
    fun::superpose(ctx.heap, &ctx.prog.aris, ctx.tid, *ctx.host, ctx.term, arg0, 0);
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 34) {
    inc_cost(ctx.heap, ctx.tid);
    let ctr_0 = get_loc(arg0, 0)/*reuse:0*/;
    let done = Ctr(_Free_, ctr_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 1);
    return false;
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 31) {
    inc_cost(ctx.heap, ctx.tid);
    let arg0_0 = load_arg(ctx.heap, arg0, 0);
    let cal_0 = get_loc(arg0, 0)/*reuse:1*/;
    link(ctx.heap, cal_0 + 0, arg0_0);
    let done = Fun(_Radix_, cal_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 1);
    return true;
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 32) {
    inc_cost(ctx.heap, ctx.tid);
    let arg0_0 = load_arg(ctx.heap, arg0, 0);
    let arg0_1 = load_arg(ctx.heap, arg0, 1);
    let cal_0 = get_loc(ctx.term, 0)/*reuse:1*/;
    link(ctx.heap, cal_0 + 0, arg0_0);
    let cal_1 = alloc(ctx.heap, ctx.tid, 1);
    link(ctx.heap, cal_1 + 0, arg0_1);
    let cal_2 = get_loc(arg0, 0)/*reuse:2*/;
    link(ctx.heap, cal_2 + 0, Fun(_ToMap_, cal_0));
    link(ctx.heap, cal_2 + 1, Fun(_ToMap_, cal_1));
    let done = Fun(_Merge_, cal_2);
    link(ctx.heap, *ctx.host, done);
    return true;
  }
  return false;
}

#[inline(always)]
pub fn _Main__visit(ctx: ReduceCtx) -> bool {
  return false;
}

#[inline(always)]
pub fn _Main__apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  if true {
    inc_cost(ctx.heap, ctx.tid);
    let cal_0 = get_loc(ctx.term, 0)/*reuse:1*/;
    link(ctx.heap, cal_0 + 0, U6O(23));
    let cal_1 = alloc(ctx.heap, ctx.tid, 1);
    link(ctx.heap, cal_1 + 0, Fun(_Gen_, cal_0));
    let cal_2 = alloc(ctx.heap, ctx.tid, 1);
    link(ctx.heap, cal_2 + 0, Fun(_Reverse_, cal_1));
    let cal_3 = alloc(ctx.heap, ctx.tid, 1);
    link(ctx.heap, cal_3 + 0, Fun(_Sort_, cal_2));
    let done = Fun(_Sum_, cal_3);
    link(ctx.heap, *ctx.host, done);
    collect(ctx.heap, &ctx.prog.aris, ctx.tid, arg0);
    return true;
  }
  return false;
}

#[inline(always)]
pub fn _Reverse__visit(ctx: ReduceCtx) -> bool {
  let mut vlen = 0;
  let vbuf = unsafe { ctx.heap.vbuf.get_unchecked(ctx.tid) };
  if !is_whnf(load_arg(ctx.heap, ctx.term, 0)) {
    unsafe { vbuf.get_unchecked(vlen) }.store(get_loc(ctx.term, 0), Ordering::Relaxed);
    vlen += 1
  }
  if vlen == 0 {
    return false;
  } else {
    let goup = ctx.redex.insert(ctx.tid, new_redex(*ctx.host, *ctx.cont, vlen as u64));
    if 0 < vlen - 1 {
      ctx.visit.push(new_visit(unsafe { vbuf.get_unchecked(0).load(Ordering::Relaxed) }, ctx.hold, goup));
    }
    *ctx.cont = goup;
    *ctx.host = unsafe { vbuf.get_unchecked(vlen - 1).load(Ordering::Relaxed) };
    return true;
  }
}

#[inline(always)]
pub fn _Reverse__apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  if get_tag(arg0) == SUP {
    fun::superpose(ctx.heap, &ctx.prog.aris, ctx.tid, *ctx.host, ctx.term, arg0, 0);
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 34) {
    inc_cost(ctx.heap, ctx.tid);
    let ctr_0 = get_loc(arg0, 0)/*reuse:0*/;
    let done = Ctr(_Null_, ctr_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 1);
    return false;
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 31) {
    inc_cost(ctx.heap, ctx.tid);
    let arg0_0 = load_arg(ctx.heap, arg0, 0);
    let ctr_0 = get_loc(arg0, 0)/*reuse:1*/;
    link(ctx.heap, ctr_0 + 0, arg0_0);
    let done = Ctr(_Leaf_, ctr_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 1);
    return false;
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 32) {
    inc_cost(ctx.heap, ctx.tid);
    let arg0_0 = load_arg(ctx.heap, arg0, 0);
    let arg0_1 = load_arg(ctx.heap, arg0, 1);
    let cal_0 = get_loc(ctx.term, 0)/*reuse:1*/;
    link(ctx.heap, cal_0 + 0, arg0_1);
    let cal_1 = alloc(ctx.heap, ctx.tid, 1);
    link(ctx.heap, cal_1 + 0, arg0_0);
    let ctr_2 = get_loc(arg0, 0)/*reuse:2*/;
    link(ctx.heap, ctr_2 + 0, Fun(_Reverse_, cal_0));
    link(ctx.heap, ctr_2 + 1, Fun(_Reverse_, cal_1));
    let done = Ctr(_Node_, ctr_2);
    link(ctx.heap, *ctx.host, done);
    return false;
  }
  return false;
}

#[inline(always)]
pub fn _Gen__visit(ctx: ReduceCtx) -> bool {
  return false;
}

#[inline(always)]
pub fn _Gen__apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  if true {
    inc_cost(ctx.heap, ctx.tid);
    let cal_0 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, cal_0 + 0, arg0);
    link(ctx.heap, cal_0 + 1, U6O(0));
    let done = Fun(_Gen_go_, cal_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 1);
    return true;
  }
  return false;
}

#[inline(always)]
pub fn _Merge__visit(ctx: ReduceCtx) -> bool {
  let mut vlen = 0;
  let vbuf = unsafe { ctx.heap.vbuf.get_unchecked(ctx.tid) };
  if !is_whnf(load_arg(ctx.heap, ctx.term, 0)) {
    unsafe { vbuf.get_unchecked(vlen) }.store(get_loc(ctx.term, 0), Ordering::Relaxed);
    vlen += 1
  }
  if !is_whnf(load_arg(ctx.heap, ctx.term, 1)) {
    unsafe { vbuf.get_unchecked(vlen) }.store(get_loc(ctx.term, 1), Ordering::Relaxed);
    vlen += 1
  }
  if vlen == 0 {
    return false;
  } else {
    let goup = ctx.redex.insert(ctx.tid, new_redex(*ctx.host, *ctx.cont, vlen as u64));
    if 0 < vlen - 1 {
      ctx.visit.push(new_visit(unsafe { vbuf.get_unchecked(0).load(Ordering::Relaxed) }, ctx.hold, goup));
    }
    if 1 < vlen - 1 {
      ctx.visit.push(new_visit(unsafe { vbuf.get_unchecked(1).load(Ordering::Relaxed) }, ctx.hold, goup));
    }
    *ctx.cont = goup;
    *ctx.host = unsafe { vbuf.get_unchecked(vlen - 1).load(Ordering::Relaxed) };
    return true;
  }
}

#[inline(always)]
pub fn _Merge__apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  let arg1 = load_arg(ctx.heap, ctx.term, 1);
  if get_tag(arg0) == SUP {
    fun::superpose(ctx.heap, &ctx.prog.aris, ctx.tid, *ctx.host, ctx.term, arg0, 0);
  }
  if get_tag(arg1) == SUP {
    fun::superpose(ctx.heap, &ctx.prog.aris, ctx.tid, *ctx.host, ctx.term, arg1, 1);
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 38) && (get_tag(arg1) == CTR && get_ext(arg1) == 38) {
    inc_cost(ctx.heap, ctx.tid);
    let ctr_0 = get_loc(arg0, 0)/*reuse:0*/;
    let done = Ctr(_Free_, ctr_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(arg1, 0), 0);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
    return false;
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 38) && (get_tag(arg1) == CTR && get_ext(arg1) == 39) {
    inc_cost(ctx.heap, ctx.tid);
    let ctr_0 = get_loc(arg0, 0)/*reuse:0*/;
    let done = Ctr(_Used_, ctr_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(arg1, 0), 0);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
    return false;
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 39) && (get_tag(arg1) == CTR && get_ext(arg1) == 38) {
    inc_cost(ctx.heap, ctx.tid);
    let ctr_0 = get_loc(arg0, 0)/*reuse:0*/;
    let done = Ctr(_Used_, ctr_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(arg1, 0), 0);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
    return false;
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 39) && (get_tag(arg1) == CTR && get_ext(arg1) == 39) {
    inc_cost(ctx.heap, ctx.tid);
    let ctr_0 = get_loc(arg0, 0)/*reuse:0*/;
    let done = Ctr(_Used_, ctr_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(arg1, 0), 0);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
    return false;
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 38) && (get_tag(arg1) == CTR && get_ext(arg1) == 2) {
    inc_cost(ctx.heap, ctx.tid);
    let arg1_0 = load_arg(ctx.heap, arg1, 0);
    let arg1_1 = load_arg(ctx.heap, arg1, 1);
    let ctr_0 = get_loc(arg1, 0)/*reuse:2*/;
    link(ctx.heap, ctr_0 + 0, arg1_0);
    link(ctx.heap, ctr_0 + 1, arg1_1);
    let done = Ctr(_Both_, ctr_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(arg0, 0), 0);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
    return false;
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 2) && (get_tag(arg1) == CTR && get_ext(arg1) == 38) {
    inc_cost(ctx.heap, ctx.tid);
    let arg0_0 = load_arg(ctx.heap, arg0, 0);
    let arg0_1 = load_arg(ctx.heap, arg0, 1);
    let ctr_0 = get_loc(arg0, 0)/*reuse:2*/;
    link(ctx.heap, ctr_0 + 0, arg0_0);
    link(ctx.heap, ctr_0 + 1, arg0_1);
    let done = Ctr(_Both_, ctr_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(arg1, 0), 0);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
    return false;
  }
  if (get_tag(arg0) == CTR && get_ext(arg0) == 2) && (get_tag(arg1) == CTR && get_ext(arg1) == 2) {
    inc_cost(ctx.heap, ctx.tid);
    let arg0_0 = load_arg(ctx.heap, arg0, 0);
    let arg0_1 = load_arg(ctx.heap, arg0, 1);
    let arg1_0 = load_arg(ctx.heap, arg1, 0);
    let arg1_1 = load_arg(ctx.heap, arg1, 1);
    let cal_0 = get_loc(arg0, 0)/*reuse:2*/;
    link(ctx.heap, cal_0 + 0, arg0_0);
    link(ctx.heap, cal_0 + 1, arg1_0);
    let cal_1 = get_loc(arg1, 0)/*reuse:2*/;
    link(ctx.heap, cal_1 + 0, arg0_1);
    link(ctx.heap, cal_1 + 1, arg1_1);
    let cal_2 = get_loc(ctx.term, 0)/*reuse:2*/;
    link(ctx.heap, cal_2 + 0, Fun(_Merge_, cal_0));
    link(ctx.heap, cal_2 + 1, Fun(_Merge_, cal_1));
    let done = Fun(_BOTH_, cal_2);
    link(ctx.heap, *ctx.host, done);
    return true;
  }
  return false;
}

#[inline(always)]
pub fn _BOTH__visit(ctx: ReduceCtx) -> bool {
  let mut vlen = 0;
  let vbuf = unsafe { ctx.heap.vbuf.get_unchecked(ctx.tid) };
  if !is_whnf(load_arg(ctx.heap, ctx.term, 0)) {
    unsafe { vbuf.get_unchecked(vlen) }.store(get_loc(ctx.term, 0), Ordering::Relaxed);
    vlen += 1
  }
  if !is_whnf(load_arg(ctx.heap, ctx.term, 1)) {
    unsafe { vbuf.get_unchecked(vlen) }.store(get_loc(ctx.term, 1), Ordering::Relaxed);
    vlen += 1
  }
  if vlen == 0 {
    return false;
  } else {
    let goup = ctx.redex.insert(ctx.tid, new_redex(*ctx.host, *ctx.cont, vlen as u64));
    if 0 < vlen - 1 {
      ctx.visit.push(new_visit(unsafe { vbuf.get_unchecked(0).load(Ordering::Relaxed) }, ctx.hold, goup));
    }
    if 1 < vlen - 1 {
      ctx.visit.push(new_visit(unsafe { vbuf.get_unchecked(1).load(Ordering::Relaxed) }, ctx.hold, goup));
    }
    *ctx.cont = goup;
    *ctx.host = unsafe { vbuf.get_unchecked(vlen - 1).load(Ordering::Relaxed) };
    return true;
  }
}

#[inline(always)]
pub fn _BOTH__apply(ctx: ReduceCtx) -> bool {
  let done = Ctr(2, get_loc(ctx.term, 0));
  link(ctx.heap, *ctx.host, done);
  return false;
}

#[inline(always)]
pub fn _Radix__visit(ctx: ReduceCtx) -> bool {
  return false;
}

#[inline(always)]
pub fn _Radix__apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  if true {
    inc_cost(ctx.heap, ctx.tid);
    let cpy_0 = arg0;
    let dp0_1;
    let dp1_2;
    if get_tag(cpy_0) == U60 || get_tag(cpy_0) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_1 = cpy_0;
      dp1_2 = cpy_0;
    } else {
      let col_3 = gen_dup(ctx.heap, ctx.tid);
      let dup_4 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_4 + 0, Era());
    link(ctx.heap, dup_4 + 1, Era());
      link(ctx.heap, dup_4 + 2, cpy_0);
      dp0_1 = Dp0(col_3, dup_4);
      dp1_2 = Dp1(col_3, dup_4);
    }
    let cpy_5 = dp0_1;
    let dp0_6;
    let dp1_7;
    if get_tag(cpy_5) == U60 || get_tag(cpy_5) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_6 = cpy_5;
      dp1_7 = cpy_5;
    } else {
      let col_8 = gen_dup(ctx.heap, ctx.tid);
      let dup_9 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_9 + 0, Era());
    link(ctx.heap, dup_9 + 1, Era());
      link(ctx.heap, dup_9 + 2, cpy_5);
      dp0_6 = Dp0(col_8, dup_9);
      dp1_7 = Dp1(col_8, dup_9);
    }
    let cpy_10 = dp1_2;
    let dp0_11;
    let dp1_12;
    if get_tag(cpy_10) == U60 || get_tag(cpy_10) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_11 = cpy_10;
      dp1_12 = cpy_10;
    } else {
      let col_13 = gen_dup(ctx.heap, ctx.tid);
      let dup_14 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_14 + 0, Era());
    link(ctx.heap, dup_14 + 1, Era());
      link(ctx.heap, dup_14 + 2, cpy_10);
      dp0_11 = Dp0(col_13, dup_14);
      dp1_12 = Dp1(col_13, dup_14);
    }
    let cpy_15 = dp0_6;
    let dp0_16;
    let dp1_17;
    if get_tag(cpy_15) == U60 || get_tag(cpy_15) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_16 = cpy_15;
      dp1_17 = cpy_15;
    } else {
      let col_18 = gen_dup(ctx.heap, ctx.tid);
      let dup_19 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_19 + 0, Era());
    link(ctx.heap, dup_19 + 1, Era());
      link(ctx.heap, dup_19 + 2, cpy_15);
      dp0_16 = Dp0(col_18, dup_19);
      dp1_17 = Dp1(col_18, dup_19);
    }
    let cpy_20 = dp1_7;
    let dp0_21;
    let dp1_22;
    if get_tag(cpy_20) == U60 || get_tag(cpy_20) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_21 = cpy_20;
      dp1_22 = cpy_20;
    } else {
      let col_23 = gen_dup(ctx.heap, ctx.tid);
      let dup_24 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_24 + 0, Era());
    link(ctx.heap, dup_24 + 1, Era());
      link(ctx.heap, dup_24 + 2, cpy_20);
      dp0_21 = Dp0(col_23, dup_24);
      dp1_22 = Dp1(col_23, dup_24);
    }
    let cpy_25 = dp0_11;
    let dp0_26;
    let dp1_27;
    if get_tag(cpy_25) == U60 || get_tag(cpy_25) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_26 = cpy_25;
      dp1_27 = cpy_25;
    } else {
      let col_28 = gen_dup(ctx.heap, ctx.tid);
      let dup_29 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_29 + 0, Era());
    link(ctx.heap, dup_29 + 1, Era());
      link(ctx.heap, dup_29 + 2, cpy_25);
      dp0_26 = Dp0(col_28, dup_29);
      dp1_27 = Dp1(col_28, dup_29);
    }
    let cpy_30 = dp1_12;
    let dp0_31;
    let dp1_32;
    if get_tag(cpy_30) == U60 || get_tag(cpy_30) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_31 = cpy_30;
      dp1_32 = cpy_30;
    } else {
      let col_33 = gen_dup(ctx.heap, ctx.tid);
      let dup_34 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_34 + 0, Era());
    link(ctx.heap, dup_34 + 1, Era());
      link(ctx.heap, dup_34 + 2, cpy_30);
      dp0_31 = Dp0(col_33, dup_34);
      dp1_32 = Dp1(col_33, dup_34);
    }
    let cpy_35 = dp0_16;
    let dp0_36;
    let dp1_37;
    if get_tag(cpy_35) == U60 || get_tag(cpy_35) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_36 = cpy_35;
      dp1_37 = cpy_35;
    } else {
      let col_38 = gen_dup(ctx.heap, ctx.tid);
      let dup_39 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_39 + 0, Era());
    link(ctx.heap, dup_39 + 1, Era());
      link(ctx.heap, dup_39 + 2, cpy_35);
      dp0_36 = Dp0(col_38, dup_39);
      dp1_37 = Dp1(col_38, dup_39);
    }
    let cpy_40 = dp1_17;
    let dp0_41;
    let dp1_42;
    if get_tag(cpy_40) == U60 || get_tag(cpy_40) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_41 = cpy_40;
      dp1_42 = cpy_40;
    } else {
      let col_43 = gen_dup(ctx.heap, ctx.tid);
      let dup_44 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_44 + 0, Era());
    link(ctx.heap, dup_44 + 1, Era());
      link(ctx.heap, dup_44 + 2, cpy_40);
      dp0_41 = Dp0(col_43, dup_44);
      dp1_42 = Dp1(col_43, dup_44);
    }
    let cpy_45 = dp0_21;
    let dp0_46;
    let dp1_47;
    if get_tag(cpy_45) == U60 || get_tag(cpy_45) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_46 = cpy_45;
      dp1_47 = cpy_45;
    } else {
      let col_48 = gen_dup(ctx.heap, ctx.tid);
      let dup_49 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_49 + 0, Era());
    link(ctx.heap, dup_49 + 1, Era());
      link(ctx.heap, dup_49 + 2, cpy_45);
      dp0_46 = Dp0(col_48, dup_49);
      dp1_47 = Dp1(col_48, dup_49);
    }
    let cpy_50 = dp1_22;
    let dp0_51;
    let dp1_52;
    if get_tag(cpy_50) == U60 || get_tag(cpy_50) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_51 = cpy_50;
      dp1_52 = cpy_50;
    } else {
      let col_53 = gen_dup(ctx.heap, ctx.tid);
      let dup_54 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_54 + 0, Era());
    link(ctx.heap, dup_54 + 1, Era());
      link(ctx.heap, dup_54 + 2, cpy_50);
      dp0_51 = Dp0(col_53, dup_54);
      dp1_52 = Dp1(col_53, dup_54);
    }
    let cpy_55 = dp0_26;
    let dp0_56;
    let dp1_57;
    if get_tag(cpy_55) == U60 || get_tag(cpy_55) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_56 = cpy_55;
      dp1_57 = cpy_55;
    } else {
      let col_58 = gen_dup(ctx.heap, ctx.tid);
      let dup_59 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_59 + 0, Era());
    link(ctx.heap, dup_59 + 1, Era());
      link(ctx.heap, dup_59 + 2, cpy_55);
      dp0_56 = Dp0(col_58, dup_59);
      dp1_57 = Dp1(col_58, dup_59);
    }
    let cpy_60 = dp1_27;
    let dp0_61;
    let dp1_62;
    if get_tag(cpy_60) == U60 || get_tag(cpy_60) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_61 = cpy_60;
      dp1_62 = cpy_60;
    } else {
      let col_63 = gen_dup(ctx.heap, ctx.tid);
      let dup_64 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_64 + 0, Era());
    link(ctx.heap, dup_64 + 1, Era());
      link(ctx.heap, dup_64 + 2, cpy_60);
      dp0_61 = Dp0(col_63, dup_64);
      dp1_62 = Dp1(col_63, dup_64);
    }
    let cpy_65 = dp0_31;
    let dp0_66;
    let dp1_67;
    if get_tag(cpy_65) == U60 || get_tag(cpy_65) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_66 = cpy_65;
      dp1_67 = cpy_65;
    } else {
      let col_68 = gen_dup(ctx.heap, ctx.tid);
      let dup_69 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_69 + 0, Era());
    link(ctx.heap, dup_69 + 1, Era());
      link(ctx.heap, dup_69 + 2, cpy_65);
      dp0_66 = Dp0(col_68, dup_69);
      dp1_67 = Dp1(col_68, dup_69);
    }
    let cpy_70 = dp1_32;
    let dp0_71;
    let dp1_72;
    if get_tag(cpy_70) == U60 || get_tag(cpy_70) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_71 = cpy_70;
      dp1_72 = cpy_70;
    } else {
      let col_73 = gen_dup(ctx.heap, ctx.tid);
      let dup_74 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_74 + 0, Era());
    link(ctx.heap, dup_74 + 1, Era());
      link(ctx.heap, dup_74 + 2, cpy_70);
      dp0_71 = Dp0(col_73, dup_74);
      dp1_72 = Dp1(col_73, dup_74);
    }
    let cpy_75 = dp0_36;
    let dp0_76;
    let dp1_77;
    if get_tag(cpy_75) == U60 || get_tag(cpy_75) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_76 = cpy_75;
      dp1_77 = cpy_75;
    } else {
      let col_78 = gen_dup(ctx.heap, ctx.tid);
      let dup_79 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_79 + 0, Era());
    link(ctx.heap, dup_79 + 1, Era());
      link(ctx.heap, dup_79 + 2, cpy_75);
      dp0_76 = Dp0(col_78, dup_79);
      dp1_77 = Dp1(col_78, dup_79);
    }
    let cpy_80 = dp1_37;
    let dp0_81;
    let dp1_82;
    if get_tag(cpy_80) == U60 || get_tag(cpy_80) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_81 = cpy_80;
      dp1_82 = cpy_80;
    } else {
      let col_83 = gen_dup(ctx.heap, ctx.tid);
      let dup_84 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_84 + 0, Era());
    link(ctx.heap, dup_84 + 1, Era());
      link(ctx.heap, dup_84 + 2, cpy_80);
      dp0_81 = Dp0(col_83, dup_84);
      dp1_82 = Dp1(col_83, dup_84);
    }
    let cpy_85 = dp0_41;
    let dp0_86;
    let dp1_87;
    if get_tag(cpy_85) == U60 || get_tag(cpy_85) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_86 = cpy_85;
      dp1_87 = cpy_85;
    } else {
      let col_88 = gen_dup(ctx.heap, ctx.tid);
      let dup_89 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_89 + 0, Era());
    link(ctx.heap, dup_89 + 1, Era());
      link(ctx.heap, dup_89 + 2, cpy_85);
      dp0_86 = Dp0(col_88, dup_89);
      dp1_87 = Dp1(col_88, dup_89);
    }
    let cpy_90 = dp1_42;
    let dp0_91;
    let dp1_92;
    if get_tag(cpy_90) == U60 || get_tag(cpy_90) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_91 = cpy_90;
      dp1_92 = cpy_90;
    } else {
      let col_93 = gen_dup(ctx.heap, ctx.tid);
      let dup_94 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_94 + 0, Era());
    link(ctx.heap, dup_94 + 1, Era());
      link(ctx.heap, dup_94 + 2, cpy_90);
      dp0_91 = Dp0(col_93, dup_94);
      dp1_92 = Dp1(col_93, dup_94);
    }
    let cpy_95 = dp0_46;
    let dp0_96;
    let dp1_97;
    if get_tag(cpy_95) == U60 || get_tag(cpy_95) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_96 = cpy_95;
      dp1_97 = cpy_95;
    } else {
      let col_98 = gen_dup(ctx.heap, ctx.tid);
      let dup_99 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_99 + 0, Era());
    link(ctx.heap, dup_99 + 1, Era());
      link(ctx.heap, dup_99 + 2, cpy_95);
      dp0_96 = Dp0(col_98, dup_99);
      dp1_97 = Dp1(col_98, dup_99);
    }
    let cpy_100 = dp1_47;
    let dp0_101;
    let dp1_102;
    if get_tag(cpy_100) == U60 || get_tag(cpy_100) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_101 = cpy_100;
      dp1_102 = cpy_100;
    } else {
      let col_103 = gen_dup(ctx.heap, ctx.tid);
      let dup_104 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_104 + 0, Era());
    link(ctx.heap, dup_104 + 1, Era());
      link(ctx.heap, dup_104 + 2, cpy_100);
      dp0_101 = Dp0(col_103, dup_104);
      dp1_102 = Dp1(col_103, dup_104);
    }
    let cpy_105 = dp0_51;
    let dp0_106;
    let dp1_107;
    if get_tag(cpy_105) == U60 || get_tag(cpy_105) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_106 = cpy_105;
      dp1_107 = cpy_105;
    } else {
      let col_108 = gen_dup(ctx.heap, ctx.tid);
      let dup_109 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_109 + 0, Era());
    link(ctx.heap, dup_109 + 1, Era());
      link(ctx.heap, dup_109 + 2, cpy_105);
      dp0_106 = Dp0(col_108, dup_109);
      dp1_107 = Dp1(col_108, dup_109);
    }
    let cpy_110 = dp1_52;
    let dp0_111;
    let dp1_112;
    if get_tag(cpy_110) == U60 || get_tag(cpy_110) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_111 = cpy_110;
      dp1_112 = cpy_110;
    } else {
      let col_113 = gen_dup(ctx.heap, ctx.tid);
      let dup_114 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_114 + 0, Era());
    link(ctx.heap, dup_114 + 1, Era());
      link(ctx.heap, dup_114 + 2, cpy_110);
      dp0_111 = Dp0(col_113, dup_114);
      dp1_112 = Dp1(col_113, dup_114);
    }
    let ctr_115 = alloc(ctx.heap, ctx.tid, 0);
    let ret_116;
    if get_tag(dp0_56) == U60 && get_tag(U6O(1)) == U60 {
      ret_116 = U6O(u60::and(get_num(dp0_56), get_num(U6O(1))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_56) == F60 && get_tag(U6O(1)) == F60 {
      ret_116 = F6O(f60::and(get_num(dp0_56), get_num(U6O(1))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_117 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_117 + 0, dp0_56);
      link(ctx.heap, op2_117 + 1, U6O(1));
      ret_116 = Op2(AND, op2_117);
    }
    let ctr_118 = alloc(ctx.heap, ctx.tid, 0);
    let ret_119;
    if get_tag(ret_116) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_116) == 0 {
        let both_120 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_120 + 0, Ctr(_Used_, ctr_115));
        link(ctx.heap, both_120 + 1, Ctr(_Free_, ctr_118));
        ret_119 = Ctr(BOTH, both_120);
      } else {
        let both_120 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_120 + 0, Ctr(_Free_, ctr_118));
        link(ctx.heap, both_120 + 1, Ctr(_Used_, ctr_115));
        ret_119 = Ctr(BOTH, both_120);
      }
    } else {
      let cal_121 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_121 + 0, ret_116);
      link(ctx.heap, cal_121 + 1, Ctr(_Used_, ctr_115));
      link(ctx.heap, cal_121 + 2, Ctr(_Free_, ctr_118));
      ret_119 = Fun(_U60_swap_, cal_121)
    }
    let ret_122;
    if get_tag(dp1_57) == U60 && get_tag(U6O(2)) == U60 {
      ret_122 = U6O(u60::and(get_num(dp1_57), get_num(U6O(2))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp1_57) == F60 && get_tag(U6O(2)) == F60 {
      ret_122 = F6O(f60::and(get_num(dp1_57), get_num(U6O(2))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_123 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_123 + 0, dp1_57);
      link(ctx.heap, op2_123 + 1, U6O(2));
      ret_122 = Op2(AND, op2_123);
    }
    let ctr_124 = alloc(ctx.heap, ctx.tid, 0);
    let ret_125;
    if get_tag(ret_122) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_122) == 0 {
        let both_126 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_126 + 0, ret_119);
        link(ctx.heap, both_126 + 1, Ctr(_Free_, ctr_124));
        ret_125 = Ctr(BOTH, both_126);
      } else {
        let both_126 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_126 + 0, Ctr(_Free_, ctr_124));
        link(ctx.heap, both_126 + 1, ret_119);
        ret_125 = Ctr(BOTH, both_126);
      }
    } else {
      let cal_127 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_127 + 0, ret_122);
      link(ctx.heap, cal_127 + 1, ret_119);
      link(ctx.heap, cal_127 + 2, Ctr(_Free_, ctr_124));
      ret_125 = Fun(_U60_swap_, cal_127)
    }
    let ret_128;
    if get_tag(dp0_61) == U60 && get_tag(U6O(4)) == U60 {
      ret_128 = U6O(u60::and(get_num(dp0_61), get_num(U6O(4))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_61) == F60 && get_tag(U6O(4)) == F60 {
      ret_128 = F6O(f60::and(get_num(dp0_61), get_num(U6O(4))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_129 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_129 + 0, dp0_61);
      link(ctx.heap, op2_129 + 1, U6O(4));
      ret_128 = Op2(AND, op2_129);
    }
    let ctr_130 = alloc(ctx.heap, ctx.tid, 0);
    let ret_131;
    if get_tag(ret_128) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_128) == 0 {
        let both_132 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_132 + 0, ret_125);
        link(ctx.heap, both_132 + 1, Ctr(_Free_, ctr_130));
        ret_131 = Ctr(BOTH, both_132);
      } else {
        let both_132 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_132 + 0, Ctr(_Free_, ctr_130));
        link(ctx.heap, both_132 + 1, ret_125);
        ret_131 = Ctr(BOTH, both_132);
      }
    } else {
      let cal_133 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_133 + 0, ret_128);
      link(ctx.heap, cal_133 + 1, ret_125);
      link(ctx.heap, cal_133 + 2, Ctr(_Free_, ctr_130));
      ret_131 = Fun(_U60_swap_, cal_133)
    }
    let ret_134;
    if get_tag(dp1_62) == U60 && get_tag(U6O(8)) == U60 {
      ret_134 = U6O(u60::and(get_num(dp1_62), get_num(U6O(8))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp1_62) == F60 && get_tag(U6O(8)) == F60 {
      ret_134 = F6O(f60::and(get_num(dp1_62), get_num(U6O(8))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_135 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_135 + 0, dp1_62);
      link(ctx.heap, op2_135 + 1, U6O(8));
      ret_134 = Op2(AND, op2_135);
    }
    let ctr_136 = alloc(ctx.heap, ctx.tid, 0);
    let ret_137;
    if get_tag(ret_134) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_134) == 0 {
        let both_138 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_138 + 0, ret_131);
        link(ctx.heap, both_138 + 1, Ctr(_Free_, ctr_136));
        ret_137 = Ctr(BOTH, both_138);
      } else {
        let both_138 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_138 + 0, Ctr(_Free_, ctr_136));
        link(ctx.heap, both_138 + 1, ret_131);
        ret_137 = Ctr(BOTH, both_138);
      }
    } else {
      let cal_139 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_139 + 0, ret_134);
      link(ctx.heap, cal_139 + 1, ret_131);
      link(ctx.heap, cal_139 + 2, Ctr(_Free_, ctr_136));
      ret_137 = Fun(_U60_swap_, cal_139)
    }
    let ret_140;
    if get_tag(dp0_66) == U60 && get_tag(U6O(16)) == U60 {
      ret_140 = U6O(u60::and(get_num(dp0_66), get_num(U6O(16))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_66) == F60 && get_tag(U6O(16)) == F60 {
      ret_140 = F6O(f60::and(get_num(dp0_66), get_num(U6O(16))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_141 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_141 + 0, dp0_66);
      link(ctx.heap, op2_141 + 1, U6O(16));
      ret_140 = Op2(AND, op2_141);
    }
    let ctr_142 = alloc(ctx.heap, ctx.tid, 0);
    let ret_143;
    if get_tag(ret_140) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_140) == 0 {
        let both_144 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_144 + 0, ret_137);
        link(ctx.heap, both_144 + 1, Ctr(_Free_, ctr_142));
        ret_143 = Ctr(BOTH, both_144);
      } else {
        let both_144 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_144 + 0, Ctr(_Free_, ctr_142));
        link(ctx.heap, both_144 + 1, ret_137);
        ret_143 = Ctr(BOTH, both_144);
      }
    } else {
      let cal_145 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_145 + 0, ret_140);
      link(ctx.heap, cal_145 + 1, ret_137);
      link(ctx.heap, cal_145 + 2, Ctr(_Free_, ctr_142));
      ret_143 = Fun(_U60_swap_, cal_145)
    }
    let ret_146;
    if get_tag(dp1_67) == U60 && get_tag(U6O(32)) == U60 {
      ret_146 = U6O(u60::and(get_num(dp1_67), get_num(U6O(32))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp1_67) == F60 && get_tag(U6O(32)) == F60 {
      ret_146 = F6O(f60::and(get_num(dp1_67), get_num(U6O(32))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_147 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_147 + 0, dp1_67);
      link(ctx.heap, op2_147 + 1, U6O(32));
      ret_146 = Op2(AND, op2_147);
    }
    let ctr_148 = alloc(ctx.heap, ctx.tid, 0);
    let ret_149;
    if get_tag(ret_146) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_146) == 0 {
        let both_150 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_150 + 0, ret_143);
        link(ctx.heap, both_150 + 1, Ctr(_Free_, ctr_148));
        ret_149 = Ctr(BOTH, both_150);
      } else {
        let both_150 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_150 + 0, Ctr(_Free_, ctr_148));
        link(ctx.heap, both_150 + 1, ret_143);
        ret_149 = Ctr(BOTH, both_150);
      }
    } else {
      let cal_151 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_151 + 0, ret_146);
      link(ctx.heap, cal_151 + 1, ret_143);
      link(ctx.heap, cal_151 + 2, Ctr(_Free_, ctr_148));
      ret_149 = Fun(_U60_swap_, cal_151)
    }
    let ret_152;
    if get_tag(dp0_71) == U60 && get_tag(U6O(64)) == U60 {
      ret_152 = U6O(u60::and(get_num(dp0_71), get_num(U6O(64))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_71) == F60 && get_tag(U6O(64)) == F60 {
      ret_152 = F6O(f60::and(get_num(dp0_71), get_num(U6O(64))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_153 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_153 + 0, dp0_71);
      link(ctx.heap, op2_153 + 1, U6O(64));
      ret_152 = Op2(AND, op2_153);
    }
    let ctr_154 = alloc(ctx.heap, ctx.tid, 0);
    let ret_155;
    if get_tag(ret_152) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_152) == 0 {
        let both_156 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_156 + 0, ret_149);
        link(ctx.heap, both_156 + 1, Ctr(_Free_, ctr_154));
        ret_155 = Ctr(BOTH, both_156);
      } else {
        let both_156 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_156 + 0, Ctr(_Free_, ctr_154));
        link(ctx.heap, both_156 + 1, ret_149);
        ret_155 = Ctr(BOTH, both_156);
      }
    } else {
      let cal_157 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_157 + 0, ret_152);
      link(ctx.heap, cal_157 + 1, ret_149);
      link(ctx.heap, cal_157 + 2, Ctr(_Free_, ctr_154));
      ret_155 = Fun(_U60_swap_, cal_157)
    }
    let ret_158;
    if get_tag(dp1_72) == U60 && get_tag(U6O(128)) == U60 {
      ret_158 = U6O(u60::and(get_num(dp1_72), get_num(U6O(128))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp1_72) == F60 && get_tag(U6O(128)) == F60 {
      ret_158 = F6O(f60::and(get_num(dp1_72), get_num(U6O(128))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_159 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_159 + 0, dp1_72);
      link(ctx.heap, op2_159 + 1, U6O(128));
      ret_158 = Op2(AND, op2_159);
    }
    let ctr_160 = alloc(ctx.heap, ctx.tid, 0);
    let ret_161;
    if get_tag(ret_158) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_158) == 0 {
        let both_162 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_162 + 0, ret_155);
        link(ctx.heap, both_162 + 1, Ctr(_Free_, ctr_160));
        ret_161 = Ctr(BOTH, both_162);
      } else {
        let both_162 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_162 + 0, Ctr(_Free_, ctr_160));
        link(ctx.heap, both_162 + 1, ret_155);
        ret_161 = Ctr(BOTH, both_162);
      }
    } else {
      let cal_163 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_163 + 0, ret_158);
      link(ctx.heap, cal_163 + 1, ret_155);
      link(ctx.heap, cal_163 + 2, Ctr(_Free_, ctr_160));
      ret_161 = Fun(_U60_swap_, cal_163)
    }
    let ret_164;
    if get_tag(dp0_76) == U60 && get_tag(U6O(256)) == U60 {
      ret_164 = U6O(u60::and(get_num(dp0_76), get_num(U6O(256))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_76) == F60 && get_tag(U6O(256)) == F60 {
      ret_164 = F6O(f60::and(get_num(dp0_76), get_num(U6O(256))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_165 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_165 + 0, dp0_76);
      link(ctx.heap, op2_165 + 1, U6O(256));
      ret_164 = Op2(AND, op2_165);
    }
    let ctr_166 = alloc(ctx.heap, ctx.tid, 0);
    let ret_167;
    if get_tag(ret_164) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_164) == 0 {
        let both_168 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_168 + 0, ret_161);
        link(ctx.heap, both_168 + 1, Ctr(_Free_, ctr_166));
        ret_167 = Ctr(BOTH, both_168);
      } else {
        let both_168 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_168 + 0, Ctr(_Free_, ctr_166));
        link(ctx.heap, both_168 + 1, ret_161);
        ret_167 = Ctr(BOTH, both_168);
      }
    } else {
      let cal_169 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_169 + 0, ret_164);
      link(ctx.heap, cal_169 + 1, ret_161);
      link(ctx.heap, cal_169 + 2, Ctr(_Free_, ctr_166));
      ret_167 = Fun(_U60_swap_, cal_169)
    }
    let ret_170;
    if get_tag(dp1_77) == U60 && get_tag(U6O(512)) == U60 {
      ret_170 = U6O(u60::and(get_num(dp1_77), get_num(U6O(512))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp1_77) == F60 && get_tag(U6O(512)) == F60 {
      ret_170 = F6O(f60::and(get_num(dp1_77), get_num(U6O(512))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_171 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_171 + 0, dp1_77);
      link(ctx.heap, op2_171 + 1, U6O(512));
      ret_170 = Op2(AND, op2_171);
    }
    let ctr_172 = alloc(ctx.heap, ctx.tid, 0);
    let ret_173;
    if get_tag(ret_170) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_170) == 0 {
        let both_174 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_174 + 0, ret_167);
        link(ctx.heap, both_174 + 1, Ctr(_Free_, ctr_172));
        ret_173 = Ctr(BOTH, both_174);
      } else {
        let both_174 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_174 + 0, Ctr(_Free_, ctr_172));
        link(ctx.heap, both_174 + 1, ret_167);
        ret_173 = Ctr(BOTH, both_174);
      }
    } else {
      let cal_175 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_175 + 0, ret_170);
      link(ctx.heap, cal_175 + 1, ret_167);
      link(ctx.heap, cal_175 + 2, Ctr(_Free_, ctr_172));
      ret_173 = Fun(_U60_swap_, cal_175)
    }
    let ret_176;
    if get_tag(dp0_81) == U60 && get_tag(U6O(1024)) == U60 {
      ret_176 = U6O(u60::and(get_num(dp0_81), get_num(U6O(1024))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_81) == F60 && get_tag(U6O(1024)) == F60 {
      ret_176 = F6O(f60::and(get_num(dp0_81), get_num(U6O(1024))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_177 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_177 + 0, dp0_81);
      link(ctx.heap, op2_177 + 1, U6O(1024));
      ret_176 = Op2(AND, op2_177);
    }
    let ctr_178 = alloc(ctx.heap, ctx.tid, 0);
    let ret_179;
    if get_tag(ret_176) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_176) == 0 {
        let both_180 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_180 + 0, ret_173);
        link(ctx.heap, both_180 + 1, Ctr(_Free_, ctr_178));
        ret_179 = Ctr(BOTH, both_180);
      } else {
        let both_180 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_180 + 0, Ctr(_Free_, ctr_178));
        link(ctx.heap, both_180 + 1, ret_173);
        ret_179 = Ctr(BOTH, both_180);
      }
    } else {
      let cal_181 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_181 + 0, ret_176);
      link(ctx.heap, cal_181 + 1, ret_173);
      link(ctx.heap, cal_181 + 2, Ctr(_Free_, ctr_178));
      ret_179 = Fun(_U60_swap_, cal_181)
    }
    let ret_182;
    if get_tag(dp1_82) == U60 && get_tag(U6O(2048)) == U60 {
      ret_182 = U6O(u60::and(get_num(dp1_82), get_num(U6O(2048))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp1_82) == F60 && get_tag(U6O(2048)) == F60 {
      ret_182 = F6O(f60::and(get_num(dp1_82), get_num(U6O(2048))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_183 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_183 + 0, dp1_82);
      link(ctx.heap, op2_183 + 1, U6O(2048));
      ret_182 = Op2(AND, op2_183);
    }
    let ctr_184 = alloc(ctx.heap, ctx.tid, 0);
    let ret_185;
    if get_tag(ret_182) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_182) == 0 {
        let both_186 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_186 + 0, ret_179);
        link(ctx.heap, both_186 + 1, Ctr(_Free_, ctr_184));
        ret_185 = Ctr(BOTH, both_186);
      } else {
        let both_186 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_186 + 0, Ctr(_Free_, ctr_184));
        link(ctx.heap, both_186 + 1, ret_179);
        ret_185 = Ctr(BOTH, both_186);
      }
    } else {
      let cal_187 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_187 + 0, ret_182);
      link(ctx.heap, cal_187 + 1, ret_179);
      link(ctx.heap, cal_187 + 2, Ctr(_Free_, ctr_184));
      ret_185 = Fun(_U60_swap_, cal_187)
    }
    let ret_188;
    if get_tag(dp0_86) == U60 && get_tag(U6O(4096)) == U60 {
      ret_188 = U6O(u60::and(get_num(dp0_86), get_num(U6O(4096))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_86) == F60 && get_tag(U6O(4096)) == F60 {
      ret_188 = F6O(f60::and(get_num(dp0_86), get_num(U6O(4096))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_189 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_189 + 0, dp0_86);
      link(ctx.heap, op2_189 + 1, U6O(4096));
      ret_188 = Op2(AND, op2_189);
    }
    let ctr_190 = alloc(ctx.heap, ctx.tid, 0);
    let ret_191;
    if get_tag(ret_188) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_188) == 0 {
        let both_192 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_192 + 0, ret_185);
        link(ctx.heap, both_192 + 1, Ctr(_Free_, ctr_190));
        ret_191 = Ctr(BOTH, both_192);
      } else {
        let both_192 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_192 + 0, Ctr(_Free_, ctr_190));
        link(ctx.heap, both_192 + 1, ret_185);
        ret_191 = Ctr(BOTH, both_192);
      }
    } else {
      let cal_193 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_193 + 0, ret_188);
      link(ctx.heap, cal_193 + 1, ret_185);
      link(ctx.heap, cal_193 + 2, Ctr(_Free_, ctr_190));
      ret_191 = Fun(_U60_swap_, cal_193)
    }
    let ret_194;
    if get_tag(dp1_87) == U60 && get_tag(U6O(8192)) == U60 {
      ret_194 = U6O(u60::and(get_num(dp1_87), get_num(U6O(8192))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp1_87) == F60 && get_tag(U6O(8192)) == F60 {
      ret_194 = F6O(f60::and(get_num(dp1_87), get_num(U6O(8192))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_195 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_195 + 0, dp1_87);
      link(ctx.heap, op2_195 + 1, U6O(8192));
      ret_194 = Op2(AND, op2_195);
    }
    let ctr_196 = alloc(ctx.heap, ctx.tid, 0);
    let ret_197;
    if get_tag(ret_194) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_194) == 0 {
        let both_198 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_198 + 0, ret_191);
        link(ctx.heap, both_198 + 1, Ctr(_Free_, ctr_196));
        ret_197 = Ctr(BOTH, both_198);
      } else {
        let both_198 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_198 + 0, Ctr(_Free_, ctr_196));
        link(ctx.heap, both_198 + 1, ret_191);
        ret_197 = Ctr(BOTH, both_198);
      }
    } else {
      let cal_199 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_199 + 0, ret_194);
      link(ctx.heap, cal_199 + 1, ret_191);
      link(ctx.heap, cal_199 + 2, Ctr(_Free_, ctr_196));
      ret_197 = Fun(_U60_swap_, cal_199)
    }
    let ret_200;
    if get_tag(dp0_91) == U60 && get_tag(U6O(16384)) == U60 {
      ret_200 = U6O(u60::and(get_num(dp0_91), get_num(U6O(16384))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_91) == F60 && get_tag(U6O(16384)) == F60 {
      ret_200 = F6O(f60::and(get_num(dp0_91), get_num(U6O(16384))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_201 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_201 + 0, dp0_91);
      link(ctx.heap, op2_201 + 1, U6O(16384));
      ret_200 = Op2(AND, op2_201);
    }
    let ctr_202 = alloc(ctx.heap, ctx.tid, 0);
    let ret_203;
    if get_tag(ret_200) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_200) == 0 {
        let both_204 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_204 + 0, ret_197);
        link(ctx.heap, both_204 + 1, Ctr(_Free_, ctr_202));
        ret_203 = Ctr(BOTH, both_204);
      } else {
        let both_204 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_204 + 0, Ctr(_Free_, ctr_202));
        link(ctx.heap, both_204 + 1, ret_197);
        ret_203 = Ctr(BOTH, both_204);
      }
    } else {
      let cal_205 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_205 + 0, ret_200);
      link(ctx.heap, cal_205 + 1, ret_197);
      link(ctx.heap, cal_205 + 2, Ctr(_Free_, ctr_202));
      ret_203 = Fun(_U60_swap_, cal_205)
    }
    let ret_206;
    if get_tag(dp1_92) == U60 && get_tag(U6O(32768)) == U60 {
      ret_206 = U6O(u60::and(get_num(dp1_92), get_num(U6O(32768))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp1_92) == F60 && get_tag(U6O(32768)) == F60 {
      ret_206 = F6O(f60::and(get_num(dp1_92), get_num(U6O(32768))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_207 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_207 + 0, dp1_92);
      link(ctx.heap, op2_207 + 1, U6O(32768));
      ret_206 = Op2(AND, op2_207);
    }
    let ctr_208 = alloc(ctx.heap, ctx.tid, 0);
    let ret_209;
    if get_tag(ret_206) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_206) == 0 {
        let both_210 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_210 + 0, ret_203);
        link(ctx.heap, both_210 + 1, Ctr(_Free_, ctr_208));
        ret_209 = Ctr(BOTH, both_210);
      } else {
        let both_210 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_210 + 0, Ctr(_Free_, ctr_208));
        link(ctx.heap, both_210 + 1, ret_203);
        ret_209 = Ctr(BOTH, both_210);
      }
    } else {
      let cal_211 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_211 + 0, ret_206);
      link(ctx.heap, cal_211 + 1, ret_203);
      link(ctx.heap, cal_211 + 2, Ctr(_Free_, ctr_208));
      ret_209 = Fun(_U60_swap_, cal_211)
    }
    let ret_212;
    if get_tag(dp0_96) == U60 && get_tag(U6O(65536)) == U60 {
      ret_212 = U6O(u60::and(get_num(dp0_96), get_num(U6O(65536))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_96) == F60 && get_tag(U6O(65536)) == F60 {
      ret_212 = F6O(f60::and(get_num(dp0_96), get_num(U6O(65536))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_213 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_213 + 0, dp0_96);
      link(ctx.heap, op2_213 + 1, U6O(65536));
      ret_212 = Op2(AND, op2_213);
    }
    let ctr_214 = alloc(ctx.heap, ctx.tid, 0);
    let ret_215;
    if get_tag(ret_212) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_212) == 0 {
        let both_216 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_216 + 0, ret_209);
        link(ctx.heap, both_216 + 1, Ctr(_Free_, ctr_214));
        ret_215 = Ctr(BOTH, both_216);
      } else {
        let both_216 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_216 + 0, Ctr(_Free_, ctr_214));
        link(ctx.heap, both_216 + 1, ret_209);
        ret_215 = Ctr(BOTH, both_216);
      }
    } else {
      let cal_217 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_217 + 0, ret_212);
      link(ctx.heap, cal_217 + 1, ret_209);
      link(ctx.heap, cal_217 + 2, Ctr(_Free_, ctr_214));
      ret_215 = Fun(_U60_swap_, cal_217)
    }
    let ret_218;
    if get_tag(dp1_97) == U60 && get_tag(U6O(131072)) == U60 {
      ret_218 = U6O(u60::and(get_num(dp1_97), get_num(U6O(131072))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp1_97) == F60 && get_tag(U6O(131072)) == F60 {
      ret_218 = F6O(f60::and(get_num(dp1_97), get_num(U6O(131072))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_219 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_219 + 0, dp1_97);
      link(ctx.heap, op2_219 + 1, U6O(131072));
      ret_218 = Op2(AND, op2_219);
    }
    let ctr_220 = alloc(ctx.heap, ctx.tid, 0);
    let ret_221;
    if get_tag(ret_218) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_218) == 0 {
        let both_222 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_222 + 0, ret_215);
        link(ctx.heap, both_222 + 1, Ctr(_Free_, ctr_220));
        ret_221 = Ctr(BOTH, both_222);
      } else {
        let both_222 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_222 + 0, Ctr(_Free_, ctr_220));
        link(ctx.heap, both_222 + 1, ret_215);
        ret_221 = Ctr(BOTH, both_222);
      }
    } else {
      let cal_223 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_223 + 0, ret_218);
      link(ctx.heap, cal_223 + 1, ret_215);
      link(ctx.heap, cal_223 + 2, Ctr(_Free_, ctr_220));
      ret_221 = Fun(_U60_swap_, cal_223)
    }
    let ret_224;
    if get_tag(dp0_101) == U60 && get_tag(U6O(262144)) == U60 {
      ret_224 = U6O(u60::and(get_num(dp0_101), get_num(U6O(262144))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_101) == F60 && get_tag(U6O(262144)) == F60 {
      ret_224 = F6O(f60::and(get_num(dp0_101), get_num(U6O(262144))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_225 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_225 + 0, dp0_101);
      link(ctx.heap, op2_225 + 1, U6O(262144));
      ret_224 = Op2(AND, op2_225);
    }
    let ctr_226 = alloc(ctx.heap, ctx.tid, 0);
    let ret_227;
    if get_tag(ret_224) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_224) == 0 {
        let both_228 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_228 + 0, ret_221);
        link(ctx.heap, both_228 + 1, Ctr(_Free_, ctr_226));
        ret_227 = Ctr(BOTH, both_228);
      } else {
        let both_228 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_228 + 0, Ctr(_Free_, ctr_226));
        link(ctx.heap, both_228 + 1, ret_221);
        ret_227 = Ctr(BOTH, both_228);
      }
    } else {
      let cal_229 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_229 + 0, ret_224);
      link(ctx.heap, cal_229 + 1, ret_221);
      link(ctx.heap, cal_229 + 2, Ctr(_Free_, ctr_226));
      ret_227 = Fun(_U60_swap_, cal_229)
    }
    let ret_230;
    if get_tag(dp1_102) == U60 && get_tag(U6O(524288)) == U60 {
      ret_230 = U6O(u60::and(get_num(dp1_102), get_num(U6O(524288))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp1_102) == F60 && get_tag(U6O(524288)) == F60 {
      ret_230 = F6O(f60::and(get_num(dp1_102), get_num(U6O(524288))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_231 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_231 + 0, dp1_102);
      link(ctx.heap, op2_231 + 1, U6O(524288));
      ret_230 = Op2(AND, op2_231);
    }
    let ctr_232 = alloc(ctx.heap, ctx.tid, 0);
    let ret_233;
    if get_tag(ret_230) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_230) == 0 {
        let both_234 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_234 + 0, ret_227);
        link(ctx.heap, both_234 + 1, Ctr(_Free_, ctr_232));
        ret_233 = Ctr(BOTH, both_234);
      } else {
        let both_234 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_234 + 0, Ctr(_Free_, ctr_232));
        link(ctx.heap, both_234 + 1, ret_227);
        ret_233 = Ctr(BOTH, both_234);
      }
    } else {
      let cal_235 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_235 + 0, ret_230);
      link(ctx.heap, cal_235 + 1, ret_227);
      link(ctx.heap, cal_235 + 2, Ctr(_Free_, ctr_232));
      ret_233 = Fun(_U60_swap_, cal_235)
    }
    let ret_236;
    if get_tag(dp0_106) == U60 && get_tag(U6O(1048576)) == U60 {
      ret_236 = U6O(u60::and(get_num(dp0_106), get_num(U6O(1048576))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_106) == F60 && get_tag(U6O(1048576)) == F60 {
      ret_236 = F6O(f60::and(get_num(dp0_106), get_num(U6O(1048576))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_237 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_237 + 0, dp0_106);
      link(ctx.heap, op2_237 + 1, U6O(1048576));
      ret_236 = Op2(AND, op2_237);
    }
    let ctr_238 = alloc(ctx.heap, ctx.tid, 0);
    let ret_239;
    if get_tag(ret_236) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_236) == 0 {
        let both_240 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_240 + 0, ret_233);
        link(ctx.heap, both_240 + 1, Ctr(_Free_, ctr_238));
        ret_239 = Ctr(BOTH, both_240);
      } else {
        let both_240 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_240 + 0, Ctr(_Free_, ctr_238));
        link(ctx.heap, both_240 + 1, ret_233);
        ret_239 = Ctr(BOTH, both_240);
      }
    } else {
      let cal_241 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_241 + 0, ret_236);
      link(ctx.heap, cal_241 + 1, ret_233);
      link(ctx.heap, cal_241 + 2, Ctr(_Free_, ctr_238));
      ret_239 = Fun(_U60_swap_, cal_241)
    }
    let ret_242;
    if get_tag(dp1_107) == U60 && get_tag(U6O(2097152)) == U60 {
      ret_242 = U6O(u60::and(get_num(dp1_107), get_num(U6O(2097152))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp1_107) == F60 && get_tag(U6O(2097152)) == F60 {
      ret_242 = F6O(f60::and(get_num(dp1_107), get_num(U6O(2097152))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_243 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_243 + 0, dp1_107);
      link(ctx.heap, op2_243 + 1, U6O(2097152));
      ret_242 = Op2(AND, op2_243);
    }
    let ctr_244 = alloc(ctx.heap, ctx.tid, 0);
    let ret_245;
    if get_tag(ret_242) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_242) == 0 {
        let both_246 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_246 + 0, ret_239);
        link(ctx.heap, both_246 + 1, Ctr(_Free_, ctr_244));
        ret_245 = Ctr(BOTH, both_246);
      } else {
        let both_246 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_246 + 0, Ctr(_Free_, ctr_244));
        link(ctx.heap, both_246 + 1, ret_239);
        ret_245 = Ctr(BOTH, both_246);
      }
    } else {
      let cal_247 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_247 + 0, ret_242);
      link(ctx.heap, cal_247 + 1, ret_239);
      link(ctx.heap, cal_247 + 2, Ctr(_Free_, ctr_244));
      ret_245 = Fun(_U60_swap_, cal_247)
    }
    let ret_248;
    if get_tag(dp0_111) == U60 && get_tag(U6O(4194304)) == U60 {
      ret_248 = U6O(u60::and(get_num(dp0_111), get_num(U6O(4194304))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp0_111) == F60 && get_tag(U6O(4194304)) == F60 {
      ret_248 = F6O(f60::and(get_num(dp0_111), get_num(U6O(4194304))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_249 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_249 + 0, dp0_111);
      link(ctx.heap, op2_249 + 1, U6O(4194304));
      ret_248 = Op2(AND, op2_249);
    }
    let ctr_250 = alloc(ctx.heap, ctx.tid, 0);
    let ret_251;
    if get_tag(ret_248) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_248) == 0 {
        let both_252 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_252 + 0, ret_245);
        link(ctx.heap, both_252 + 1, Ctr(_Free_, ctr_250));
        ret_251 = Ctr(BOTH, both_252);
      } else {
        let both_252 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_252 + 0, Ctr(_Free_, ctr_250));
        link(ctx.heap, both_252 + 1, ret_245);
        ret_251 = Ctr(BOTH, both_252);
      }
    } else {
      let cal_253 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_253 + 0, ret_248);
      link(ctx.heap, cal_253 + 1, ret_245);
      link(ctx.heap, cal_253 + 2, Ctr(_Free_, ctr_250));
      ret_251 = Fun(_U60_swap_, cal_253)
    }
    let ret_254;
    if get_tag(dp1_112) == U60 && get_tag(U6O(8388608)) == U60 {
      ret_254 = U6O(u60::and(get_num(dp1_112), get_num(U6O(8388608))));
      inc_cost(ctx.heap, ctx.tid);
    } else if get_tag(dp1_112) == F60 && get_tag(U6O(8388608)) == F60 {
      ret_254 = F6O(f60::and(get_num(dp1_112), get_num(U6O(8388608))));
      inc_cost(ctx.heap, ctx.tid);
    } else {
      let op2_255 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, op2_255 + 0, dp1_112);
      link(ctx.heap, op2_255 + 1, U6O(8388608));
      ret_254 = Op2(AND, op2_255);
    }
    let ctr_256 = alloc(ctx.heap, ctx.tid, 0);
    let ret_257;
    if get_tag(ret_254) == U60 {
      inc_cost(ctx.heap, ctx.tid);
      if get_num(ret_254) == 0 {
        let both_258 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_258 + 0, ret_251);
        link(ctx.heap, both_258 + 1, Ctr(_Free_, ctr_256));
        ret_257 = Ctr(BOTH, both_258);
      } else {
        let both_258 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, both_258 + 0, Ctr(_Free_, ctr_256));
        link(ctx.heap, both_258 + 1, ret_251);
        ret_257 = Ctr(BOTH, both_258);
      }
    } else {
      let cal_259 = alloc(ctx.heap, ctx.tid, 3);
      link(ctx.heap, cal_259 + 0, ret_254);
      link(ctx.heap, cal_259 + 1, ret_251);
      link(ctx.heap, cal_259 + 2, Ctr(_Free_, ctr_256));
      ret_257 = Fun(_U60_swap_, cal_259)
    }
    let done = ret_257;
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 1);
    return true;
  }
  return false;
}

