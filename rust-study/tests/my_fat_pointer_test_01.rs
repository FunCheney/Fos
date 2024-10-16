//! https://rustmagazine.github.io/rust_magazine_2021/chapter_4/ant_trait.html

use std::fmt::{Debug, Error, Formatter};
use std::intrinsics::{ transmute};
use std::mem::size_of_val;

struct  FmtWrap<'a, T: 'a>(&'a fn(&T, &mut Formatter) -> Result<(), Error>, &'a T);

impl<'a, T> Debug for FmtWrap<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.0(self.1, f)
    }
}
///
/// pub fn get_vtable<'a, 'tcx>(cx: &CodegenCx<'a, 'tcx>,
///                             ty: Ty<'tcx>,
///                             trait_ref: Option<ty::PolyExistentialTraitRef<'tcx>>)
///                             -> ValueRef
/// {
///     let tcx = cx.tcx;
///
///     debug!("get_vtable(ty={:?}, trait_ref={:?})", ty, trait_ref);
///
///     // Check the cache.
///     if let Some(&val) = cx.vtables.borrow().get(&(ty, trait_ref)) {
///         return val;
///     }
///
///     // Not in the cache. Build it.
///     let nullptr = C_null(Type::i8p(cx));
///
///     let (size, align) = cx.size_and_align_of(ty);
///
///     // 所以在任何方法之前，虚表中有三个值（大小为 usize）
///     let mut components: Vec<_> = [
///         callee::get_fn(cx, monomorphize::resolve_drop_in_place(cx.tcx, ty)),
///         C_usize(cx, size.bytes()),
///         C_usize(cx, align.abi())
///     ].iter().cloned().collect();

///     if let Some(trait_ref) = trait_ref {
///         let trait_ref = trait_ref.with_self_ty(tcx, ty);
///         let methods = tcx.vtable_methods(trait_ref);
///         let methods = methods.iter().cloned().map(|opt_mth| {
///             opt_mth.map_or(nullptr, |(def_id, substs)| {
///                 callee::resolve_and_get_fn(cx, def_id, substs)
///             })
///         });
///         components.extend(methods);
///     }
///
///     let vtable_const = C_struct(cx, &components, false);
///     let align = cx.data_layout().pointer_align;
///     let vtable = consts::addr_of(cx, vtable_const, align, "vtable");
///
///     debuginfo::create_vtable_metadata(cx, ty, vtable);
///
///     cx.vtables.borrow_mut().insert((ty, trait_ref), vtable);
///     vtable
/// }
///
#[test]
fn test_01() {
    let  v = vec![1,2,3,4];
    let c: &dyn Debug = &v;

    let (_, vtable) = unsafe { transmute::<_, (usize, usize)>(c) };
    //  the function get_vtable in rust/src/librustc_codegen_llvm/meth.rs seems to hold the answer
    let fmt = unsafe { &*((vtable as *const fn(&Vec<u64>, &mut Formatter) -> Result<(), Error>).offset(3)) };
    // [1, 2, 3, 4]
    println!("{:?}", FmtWrap(fmt, &v));

    // the first pointer is apparently needed for std::ptr::drop_in_place()
    // [4300804048, 24, 8]
    // 24 是一个 Vec 的大小，而 8 是它的对齐值。
    // 这是有道理的；24 是 8 的 3 倍，并且（在 64 位架构上）Vec 应该由三个 64 位 / 8 字节的值组成，因此也应该具有 8 字节的对齐值。
    println!("{:?}", unsafe { &*(vtable as *const [usize; 3]) });

    println!("{}", v.len());
    // Vec<T> 的内部结构通常包含以下三个字段：
    //
    // 指针：指向存储在堆上的数据的指针（*mut T）。 在大多数现代 64 位系统中，指针的大小是 8 字节。
    // 长度：当前向量中的元素数量（usize）。 usize 类型的大小也是 8 字节。
    // 容量：向量的总容量，表示能够存储多少个元素（usize）。 usize 类型的大小同样是 8 字节。
    println!("{}", size_of_val(&v))
}