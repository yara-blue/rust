use rustc_middle::ty::{self, Ty};

use crate::const_eval::CompileTimeMachine;
use crate::interpret::{InterpCx, OpTy, Projectable};

fn print_numeric<'tcx>(arg: OpTy<'tcx>, ecx: &mut InterpCx<'tcx, CompileTimeMachine<'tcx>>) {
    let num = ecx.read_scalar(&arg).unwrap();
    let num = match num.size().bytes() {
        1 => num.to_u8().unwrap() as usize,
        2 => num.to_u16().unwrap() as usize,
        4 => num.to_u32().unwrap() as usize,
        8 => num.to_u64().unwrap() as usize,
        16 => num.to_u128().unwrap() as usize,
        _ => unreachable!(),
    };
    eprint!("{num}")
}

fn print_str<'tcx>(arg: OpTy<'tcx>, ecx: &mut InterpCx<'tcx, CompileTimeMachine<'tcx>>) {
    let mplace = arg.as_mplace_or_imm().unwrap_left();
    let s = ecx.read_str(&mplace).unwrap();
    eprint!("{s}")
}

pub(crate) fn print<'tcx>(
    ty: Ty<'_>,
    args: &[crate::interpret::OpTy<'tcx>],
    ecx: &mut InterpCx<'tcx, CompileTimeMachine<'tcx>>,
) {
    let arg = args.first().unwrap();
    let (arg, ty) = if let ty::Ref(_, ty, _) = ty.kind() {
        let arg = ecx.deref_pointer(arg).unwrap();
        (arg.to_op(ecx).unwrap(), ty.clone())
    } else {
        (arg.clone(), ty)
    };

    if ty.is_numeric() {
        print_numeric(arg, ecx);
    } else if ty.is_str() {
        print_str(arg, ecx);
    }
}
