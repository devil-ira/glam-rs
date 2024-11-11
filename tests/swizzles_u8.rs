// Generated by swizzlegen. Do not edit.
#[macro_use]
mod support;
use glam::*;

glam_test!(test_u8vec4_swizzles, {
    let v = u8vec4(1_u8, 2_u8, 3_u8, 4_u8);
    assert_eq!(v, v.xyzw());
    assert_eq!(v.xxxx(), u8vec4(1_u8, 1_u8, 1_u8, 1_u8));
    assert_eq!(v.xxxy(), u8vec4(1_u8, 1_u8, 1_u8, 2_u8));
    assert_eq!(v.xxxz(), u8vec4(1_u8, 1_u8, 1_u8, 3_u8));
    assert_eq!(v.xxxw(), u8vec4(1_u8, 1_u8, 1_u8, 4_u8));
    assert_eq!(v.xxyx(), u8vec4(1_u8, 1_u8, 2_u8, 1_u8));
    assert_eq!(v.xxyy(), u8vec4(1_u8, 1_u8, 2_u8, 2_u8));
    assert_eq!(v.xxyz(), u8vec4(1_u8, 1_u8, 2_u8, 3_u8));
    assert_eq!(v.xxyw(), u8vec4(1_u8, 1_u8, 2_u8, 4_u8));
    assert_eq!(v.xxzx(), u8vec4(1_u8, 1_u8, 3_u8, 1_u8));
    assert_eq!(v.xxzy(), u8vec4(1_u8, 1_u8, 3_u8, 2_u8));
    assert_eq!(v.xxzz(), u8vec4(1_u8, 1_u8, 3_u8, 3_u8));
    assert_eq!(v.xxzw(), u8vec4(1_u8, 1_u8, 3_u8, 4_u8));
    assert_eq!(v.xxwx(), u8vec4(1_u8, 1_u8, 4_u8, 1_u8));
    assert_eq!(v.xxwy(), u8vec4(1_u8, 1_u8, 4_u8, 2_u8));
    assert_eq!(v.xxwz(), u8vec4(1_u8, 1_u8, 4_u8, 3_u8));
    assert_eq!(v.xxww(), u8vec4(1_u8, 1_u8, 4_u8, 4_u8));
    assert_eq!(v.xyxx(), u8vec4(1_u8, 2_u8, 1_u8, 1_u8));
    assert_eq!(v.xyxy(), u8vec4(1_u8, 2_u8, 1_u8, 2_u8));
    assert_eq!(v.xyxz(), u8vec4(1_u8, 2_u8, 1_u8, 3_u8));
    assert_eq!(v.xyxw(), u8vec4(1_u8, 2_u8, 1_u8, 4_u8));
    assert_eq!(v.xyyx(), u8vec4(1_u8, 2_u8, 2_u8, 1_u8));
    assert_eq!(v.xyyy(), u8vec4(1_u8, 2_u8, 2_u8, 2_u8));
    assert_eq!(v.xyyz(), u8vec4(1_u8, 2_u8, 2_u8, 3_u8));
    assert_eq!(v.xyyw(), u8vec4(1_u8, 2_u8, 2_u8, 4_u8));
    assert_eq!(v.xyzx(), u8vec4(1_u8, 2_u8, 3_u8, 1_u8));
    assert_eq!(v.xyzy(), u8vec4(1_u8, 2_u8, 3_u8, 2_u8));
    assert_eq!(v.xyzz(), u8vec4(1_u8, 2_u8, 3_u8, 3_u8));
    assert_eq!(v.xywx(), u8vec4(1_u8, 2_u8, 4_u8, 1_u8));
    assert_eq!(v.xywy(), u8vec4(1_u8, 2_u8, 4_u8, 2_u8));
    assert_eq!(v.xywz(), u8vec4(1_u8, 2_u8, 4_u8, 3_u8));
    assert_eq!(v.xyww(), u8vec4(1_u8, 2_u8, 4_u8, 4_u8));
    assert_eq!(v.xzxx(), u8vec4(1_u8, 3_u8, 1_u8, 1_u8));
    assert_eq!(v.xzxy(), u8vec4(1_u8, 3_u8, 1_u8, 2_u8));
    assert_eq!(v.xzxz(), u8vec4(1_u8, 3_u8, 1_u8, 3_u8));
    assert_eq!(v.xzxw(), u8vec4(1_u8, 3_u8, 1_u8, 4_u8));
    assert_eq!(v.xzyx(), u8vec4(1_u8, 3_u8, 2_u8, 1_u8));
    assert_eq!(v.xzyy(), u8vec4(1_u8, 3_u8, 2_u8, 2_u8));
    assert_eq!(v.xzyz(), u8vec4(1_u8, 3_u8, 2_u8, 3_u8));
    assert_eq!(v.xzyw(), u8vec4(1_u8, 3_u8, 2_u8, 4_u8));
    assert_eq!(v.xzzx(), u8vec4(1_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.xzzy(), u8vec4(1_u8, 3_u8, 3_u8, 2_u8));
    assert_eq!(v.xzzz(), u8vec4(1_u8, 3_u8, 3_u8, 3_u8));
    assert_eq!(v.xzzw(), u8vec4(1_u8, 3_u8, 3_u8, 4_u8));
    assert_eq!(v.xzwx(), u8vec4(1_u8, 3_u8, 4_u8, 1_u8));
    assert_eq!(v.xzwy(), u8vec4(1_u8, 3_u8, 4_u8, 2_u8));
    assert_eq!(v.xzwz(), u8vec4(1_u8, 3_u8, 4_u8, 3_u8));
    assert_eq!(v.xzww(), u8vec4(1_u8, 3_u8, 4_u8, 4_u8));
    assert_eq!(v.xwxx(), u8vec4(1_u8, 4_u8, 1_u8, 1_u8));
    assert_eq!(v.xwxy(), u8vec4(1_u8, 4_u8, 1_u8, 2_u8));
    assert_eq!(v.xwxz(), u8vec4(1_u8, 4_u8, 1_u8, 3_u8));
    assert_eq!(v.xwxw(), u8vec4(1_u8, 4_u8, 1_u8, 4_u8));
    assert_eq!(v.xwyx(), u8vec4(1_u8, 4_u8, 2_u8, 1_u8));
    assert_eq!(v.xwyy(), u8vec4(1_u8, 4_u8, 2_u8, 2_u8));
    assert_eq!(v.xwyz(), u8vec4(1_u8, 4_u8, 2_u8, 3_u8));
    assert_eq!(v.xwyw(), u8vec4(1_u8, 4_u8, 2_u8, 4_u8));
    assert_eq!(v.xwzx(), u8vec4(1_u8, 4_u8, 3_u8, 1_u8));
    assert_eq!(v.xwzy(), u8vec4(1_u8, 4_u8, 3_u8, 2_u8));
    assert_eq!(v.xwzz(), u8vec4(1_u8, 4_u8, 3_u8, 3_u8));
    assert_eq!(v.xwzw(), u8vec4(1_u8, 4_u8, 3_u8, 4_u8));
    assert_eq!(v.xwwx(), u8vec4(1_u8, 4_u8, 4_u8, 1_u8));
    assert_eq!(v.xwwy(), u8vec4(1_u8, 4_u8, 4_u8, 2_u8));
    assert_eq!(v.xwwz(), u8vec4(1_u8, 4_u8, 4_u8, 3_u8));
    assert_eq!(v.xwww(), u8vec4(1_u8, 4_u8, 4_u8, 4_u8));
    assert_eq!(v.yxxx(), u8vec4(2_u8, 1_u8, 1_u8, 1_u8));
    assert_eq!(v.yxxy(), u8vec4(2_u8, 1_u8, 1_u8, 2_u8));
    assert_eq!(v.yxxz(), u8vec4(2_u8, 1_u8, 1_u8, 3_u8));
    assert_eq!(v.yxxw(), u8vec4(2_u8, 1_u8, 1_u8, 4_u8));
    assert_eq!(v.yxyx(), u8vec4(2_u8, 1_u8, 2_u8, 1_u8));
    assert_eq!(v.yxyy(), u8vec4(2_u8, 1_u8, 2_u8, 2_u8));
    assert_eq!(v.yxyz(), u8vec4(2_u8, 1_u8, 2_u8, 3_u8));
    assert_eq!(v.yxyw(), u8vec4(2_u8, 1_u8, 2_u8, 4_u8));
    assert_eq!(v.yxzx(), u8vec4(2_u8, 1_u8, 3_u8, 1_u8));
    assert_eq!(v.yxzy(), u8vec4(2_u8, 1_u8, 3_u8, 2_u8));
    assert_eq!(v.yxzz(), u8vec4(2_u8, 1_u8, 3_u8, 3_u8));
    assert_eq!(v.yxzw(), u8vec4(2_u8, 1_u8, 3_u8, 4_u8));
    assert_eq!(v.yxwx(), u8vec4(2_u8, 1_u8, 4_u8, 1_u8));
    assert_eq!(v.yxwy(), u8vec4(2_u8, 1_u8, 4_u8, 2_u8));
    assert_eq!(v.yxwz(), u8vec4(2_u8, 1_u8, 4_u8, 3_u8));
    assert_eq!(v.yxww(), u8vec4(2_u8, 1_u8, 4_u8, 4_u8));
    assert_eq!(v.yyxx(), u8vec4(2_u8, 2_u8, 1_u8, 1_u8));
    assert_eq!(v.yyxy(), u8vec4(2_u8, 2_u8, 1_u8, 2_u8));
    assert_eq!(v.yyxz(), u8vec4(2_u8, 2_u8, 1_u8, 3_u8));
    assert_eq!(v.yyxw(), u8vec4(2_u8, 2_u8, 1_u8, 4_u8));
    assert_eq!(v.yyyx(), u8vec4(2_u8, 2_u8, 2_u8, 1_u8));
    assert_eq!(v.yyyy(), u8vec4(2_u8, 2_u8, 2_u8, 2_u8));
    assert_eq!(v.yyyz(), u8vec4(2_u8, 2_u8, 2_u8, 3_u8));
    assert_eq!(v.yyyw(), u8vec4(2_u8, 2_u8, 2_u8, 4_u8));
    assert_eq!(v.yyzx(), u8vec4(2_u8, 2_u8, 3_u8, 1_u8));
    assert_eq!(v.yyzy(), u8vec4(2_u8, 2_u8, 3_u8, 2_u8));
    assert_eq!(v.yyzz(), u8vec4(2_u8, 2_u8, 3_u8, 3_u8));
    assert_eq!(v.yyzw(), u8vec4(2_u8, 2_u8, 3_u8, 4_u8));
    assert_eq!(v.yywx(), u8vec4(2_u8, 2_u8, 4_u8, 1_u8));
    assert_eq!(v.yywy(), u8vec4(2_u8, 2_u8, 4_u8, 2_u8));
    assert_eq!(v.yywz(), u8vec4(2_u8, 2_u8, 4_u8, 3_u8));
    assert_eq!(v.yyww(), u8vec4(2_u8, 2_u8, 4_u8, 4_u8));
    assert_eq!(v.yzxx(), u8vec4(2_u8, 3_u8, 1_u8, 1_u8));
    assert_eq!(v.yzxy(), u8vec4(2_u8, 3_u8, 1_u8, 2_u8));
    assert_eq!(v.yzxz(), u8vec4(2_u8, 3_u8, 1_u8, 3_u8));
    assert_eq!(v.yzxw(), u8vec4(2_u8, 3_u8, 1_u8, 4_u8));
    assert_eq!(v.yzyx(), u8vec4(2_u8, 3_u8, 2_u8, 1_u8));
    assert_eq!(v.yzyy(), u8vec4(2_u8, 3_u8, 2_u8, 2_u8));
    assert_eq!(v.yzyz(), u8vec4(2_u8, 3_u8, 2_u8, 3_u8));
    assert_eq!(v.yzyw(), u8vec4(2_u8, 3_u8, 2_u8, 4_u8));
    assert_eq!(v.yzzx(), u8vec4(2_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.yzzy(), u8vec4(2_u8, 3_u8, 3_u8, 2_u8));
    assert_eq!(v.yzzz(), u8vec4(2_u8, 3_u8, 3_u8, 3_u8));
    assert_eq!(v.yzzw(), u8vec4(2_u8, 3_u8, 3_u8, 4_u8));
    assert_eq!(v.yzwx(), u8vec4(2_u8, 3_u8, 4_u8, 1_u8));
    assert_eq!(v.yzwy(), u8vec4(2_u8, 3_u8, 4_u8, 2_u8));
    assert_eq!(v.yzwz(), u8vec4(2_u8, 3_u8, 4_u8, 3_u8));
    assert_eq!(v.yzww(), u8vec4(2_u8, 3_u8, 4_u8, 4_u8));
    assert_eq!(v.ywxx(), u8vec4(2_u8, 4_u8, 1_u8, 1_u8));
    assert_eq!(v.ywxy(), u8vec4(2_u8, 4_u8, 1_u8, 2_u8));
    assert_eq!(v.ywxz(), u8vec4(2_u8, 4_u8, 1_u8, 3_u8));
    assert_eq!(v.ywxw(), u8vec4(2_u8, 4_u8, 1_u8, 4_u8));
    assert_eq!(v.ywyx(), u8vec4(2_u8, 4_u8, 2_u8, 1_u8));
    assert_eq!(v.ywyy(), u8vec4(2_u8, 4_u8, 2_u8, 2_u8));
    assert_eq!(v.ywyz(), u8vec4(2_u8, 4_u8, 2_u8, 3_u8));
    assert_eq!(v.ywyw(), u8vec4(2_u8, 4_u8, 2_u8, 4_u8));
    assert_eq!(v.ywzx(), u8vec4(2_u8, 4_u8, 3_u8, 1_u8));
    assert_eq!(v.ywzy(), u8vec4(2_u8, 4_u8, 3_u8, 2_u8));
    assert_eq!(v.ywzz(), u8vec4(2_u8, 4_u8, 3_u8, 3_u8));
    assert_eq!(v.ywzw(), u8vec4(2_u8, 4_u8, 3_u8, 4_u8));
    assert_eq!(v.ywwx(), u8vec4(2_u8, 4_u8, 4_u8, 1_u8));
    assert_eq!(v.ywwy(), u8vec4(2_u8, 4_u8, 4_u8, 2_u8));
    assert_eq!(v.ywwz(), u8vec4(2_u8, 4_u8, 4_u8, 3_u8));
    assert_eq!(v.ywww(), u8vec4(2_u8, 4_u8, 4_u8, 4_u8));
    assert_eq!(v.zxxx(), u8vec4(3_u8, 1_u8, 1_u8, 1_u8));
    assert_eq!(v.zxxy(), u8vec4(3_u8, 1_u8, 1_u8, 2_u8));
    assert_eq!(v.zxxz(), u8vec4(3_u8, 1_u8, 1_u8, 3_u8));
    assert_eq!(v.zxxw(), u8vec4(3_u8, 1_u8, 1_u8, 4_u8));
    assert_eq!(v.zxyx(), u8vec4(3_u8, 1_u8, 2_u8, 1_u8));
    assert_eq!(v.zxyy(), u8vec4(3_u8, 1_u8, 2_u8, 2_u8));
    assert_eq!(v.zxyz(), u8vec4(3_u8, 1_u8, 2_u8, 3_u8));
    assert_eq!(v.zxyw(), u8vec4(3_u8, 1_u8, 2_u8, 4_u8));
    assert_eq!(v.zxzx(), u8vec4(3_u8, 1_u8, 3_u8, 1_u8));
    assert_eq!(v.zxzy(), u8vec4(3_u8, 1_u8, 3_u8, 2_u8));
    assert_eq!(v.zxzz(), u8vec4(3_u8, 1_u8, 3_u8, 3_u8));
    assert_eq!(v.zxzw(), u8vec4(3_u8, 1_u8, 3_u8, 4_u8));
    assert_eq!(v.zxwx(), u8vec4(3_u8, 1_u8, 4_u8, 1_u8));
    assert_eq!(v.zxwy(), u8vec4(3_u8, 1_u8, 4_u8, 2_u8));
    assert_eq!(v.zxwz(), u8vec4(3_u8, 1_u8, 4_u8, 3_u8));
    assert_eq!(v.zxww(), u8vec4(3_u8, 1_u8, 4_u8, 4_u8));
    assert_eq!(v.zyxx(), u8vec4(3_u8, 2_u8, 1_u8, 1_u8));
    assert_eq!(v.zyxy(), u8vec4(3_u8, 2_u8, 1_u8, 2_u8));
    assert_eq!(v.zyxz(), u8vec4(3_u8, 2_u8, 1_u8, 3_u8));
    assert_eq!(v.zyxw(), u8vec4(3_u8, 2_u8, 1_u8, 4_u8));
    assert_eq!(v.zyyx(), u8vec4(3_u8, 2_u8, 2_u8, 1_u8));
    assert_eq!(v.zyyy(), u8vec4(3_u8, 2_u8, 2_u8, 2_u8));
    assert_eq!(v.zyyz(), u8vec4(3_u8, 2_u8, 2_u8, 3_u8));
    assert_eq!(v.zyyw(), u8vec4(3_u8, 2_u8, 2_u8, 4_u8));
    assert_eq!(v.zyzx(), u8vec4(3_u8, 2_u8, 3_u8, 1_u8));
    assert_eq!(v.zyzy(), u8vec4(3_u8, 2_u8, 3_u8, 2_u8));
    assert_eq!(v.zyzz(), u8vec4(3_u8, 2_u8, 3_u8, 3_u8));
    assert_eq!(v.zyzw(), u8vec4(3_u8, 2_u8, 3_u8, 4_u8));
    assert_eq!(v.zywx(), u8vec4(3_u8, 2_u8, 4_u8, 1_u8));
    assert_eq!(v.zywy(), u8vec4(3_u8, 2_u8, 4_u8, 2_u8));
    assert_eq!(v.zywz(), u8vec4(3_u8, 2_u8, 4_u8, 3_u8));
    assert_eq!(v.zyww(), u8vec4(3_u8, 2_u8, 4_u8, 4_u8));
    assert_eq!(v.zzxx(), u8vec4(3_u8, 3_u8, 1_u8, 1_u8));
    assert_eq!(v.zzxy(), u8vec4(3_u8, 3_u8, 1_u8, 2_u8));
    assert_eq!(v.zzxz(), u8vec4(3_u8, 3_u8, 1_u8, 3_u8));
    assert_eq!(v.zzxw(), u8vec4(3_u8, 3_u8, 1_u8, 4_u8));
    assert_eq!(v.zzyx(), u8vec4(3_u8, 3_u8, 2_u8, 1_u8));
    assert_eq!(v.zzyy(), u8vec4(3_u8, 3_u8, 2_u8, 2_u8));
    assert_eq!(v.zzyz(), u8vec4(3_u8, 3_u8, 2_u8, 3_u8));
    assert_eq!(v.zzyw(), u8vec4(3_u8, 3_u8, 2_u8, 4_u8));
    assert_eq!(v.zzzx(), u8vec4(3_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.zzzy(), u8vec4(3_u8, 3_u8, 3_u8, 2_u8));
    assert_eq!(v.zzzz(), u8vec4(3_u8, 3_u8, 3_u8, 3_u8));
    assert_eq!(v.zzzw(), u8vec4(3_u8, 3_u8, 3_u8, 4_u8));
    assert_eq!(v.zzwx(), u8vec4(3_u8, 3_u8, 4_u8, 1_u8));
    assert_eq!(v.zzwy(), u8vec4(3_u8, 3_u8, 4_u8, 2_u8));
    assert_eq!(v.zzwz(), u8vec4(3_u8, 3_u8, 4_u8, 3_u8));
    assert_eq!(v.zzww(), u8vec4(3_u8, 3_u8, 4_u8, 4_u8));
    assert_eq!(v.zwxx(), u8vec4(3_u8, 4_u8, 1_u8, 1_u8));
    assert_eq!(v.zwxy(), u8vec4(3_u8, 4_u8, 1_u8, 2_u8));
    assert_eq!(v.zwxz(), u8vec4(3_u8, 4_u8, 1_u8, 3_u8));
    assert_eq!(v.zwxw(), u8vec4(3_u8, 4_u8, 1_u8, 4_u8));
    assert_eq!(v.zwyx(), u8vec4(3_u8, 4_u8, 2_u8, 1_u8));
    assert_eq!(v.zwyy(), u8vec4(3_u8, 4_u8, 2_u8, 2_u8));
    assert_eq!(v.zwyz(), u8vec4(3_u8, 4_u8, 2_u8, 3_u8));
    assert_eq!(v.zwyw(), u8vec4(3_u8, 4_u8, 2_u8, 4_u8));
    assert_eq!(v.zwzx(), u8vec4(3_u8, 4_u8, 3_u8, 1_u8));
    assert_eq!(v.zwzy(), u8vec4(3_u8, 4_u8, 3_u8, 2_u8));
    assert_eq!(v.zwzz(), u8vec4(3_u8, 4_u8, 3_u8, 3_u8));
    assert_eq!(v.zwzw(), u8vec4(3_u8, 4_u8, 3_u8, 4_u8));
    assert_eq!(v.zwwx(), u8vec4(3_u8, 4_u8, 4_u8, 1_u8));
    assert_eq!(v.zwwy(), u8vec4(3_u8, 4_u8, 4_u8, 2_u8));
    assert_eq!(v.zwwz(), u8vec4(3_u8, 4_u8, 4_u8, 3_u8));
    assert_eq!(v.zwww(), u8vec4(3_u8, 4_u8, 4_u8, 4_u8));
    assert_eq!(v.wxxx(), u8vec4(4_u8, 1_u8, 1_u8, 1_u8));
    assert_eq!(v.wxxy(), u8vec4(4_u8, 1_u8, 1_u8, 2_u8));
    assert_eq!(v.wxxz(), u8vec4(4_u8, 1_u8, 1_u8, 3_u8));
    assert_eq!(v.wxxw(), u8vec4(4_u8, 1_u8, 1_u8, 4_u8));
    assert_eq!(v.wxyx(), u8vec4(4_u8, 1_u8, 2_u8, 1_u8));
    assert_eq!(v.wxyy(), u8vec4(4_u8, 1_u8, 2_u8, 2_u8));
    assert_eq!(v.wxyz(), u8vec4(4_u8, 1_u8, 2_u8, 3_u8));
    assert_eq!(v.wxyw(), u8vec4(4_u8, 1_u8, 2_u8, 4_u8));
    assert_eq!(v.wxzx(), u8vec4(4_u8, 1_u8, 3_u8, 1_u8));
    assert_eq!(v.wxzy(), u8vec4(4_u8, 1_u8, 3_u8, 2_u8));
    assert_eq!(v.wxzz(), u8vec4(4_u8, 1_u8, 3_u8, 3_u8));
    assert_eq!(v.wxzw(), u8vec4(4_u8, 1_u8, 3_u8, 4_u8));
    assert_eq!(v.wxwx(), u8vec4(4_u8, 1_u8, 4_u8, 1_u8));
    assert_eq!(v.wxwy(), u8vec4(4_u8, 1_u8, 4_u8, 2_u8));
    assert_eq!(v.wxwz(), u8vec4(4_u8, 1_u8, 4_u8, 3_u8));
    assert_eq!(v.wxww(), u8vec4(4_u8, 1_u8, 4_u8, 4_u8));
    assert_eq!(v.wyxx(), u8vec4(4_u8, 2_u8, 1_u8, 1_u8));
    assert_eq!(v.wyxy(), u8vec4(4_u8, 2_u8, 1_u8, 2_u8));
    assert_eq!(v.wyxz(), u8vec4(4_u8, 2_u8, 1_u8, 3_u8));
    assert_eq!(v.wyxw(), u8vec4(4_u8, 2_u8, 1_u8, 4_u8));
    assert_eq!(v.wyyx(), u8vec4(4_u8, 2_u8, 2_u8, 1_u8));
    assert_eq!(v.wyyy(), u8vec4(4_u8, 2_u8, 2_u8, 2_u8));
    assert_eq!(v.wyyz(), u8vec4(4_u8, 2_u8, 2_u8, 3_u8));
    assert_eq!(v.wyyw(), u8vec4(4_u8, 2_u8, 2_u8, 4_u8));
    assert_eq!(v.wyzx(), u8vec4(4_u8, 2_u8, 3_u8, 1_u8));
    assert_eq!(v.wyzy(), u8vec4(4_u8, 2_u8, 3_u8, 2_u8));
    assert_eq!(v.wyzz(), u8vec4(4_u8, 2_u8, 3_u8, 3_u8));
    assert_eq!(v.wyzw(), u8vec4(4_u8, 2_u8, 3_u8, 4_u8));
    assert_eq!(v.wywx(), u8vec4(4_u8, 2_u8, 4_u8, 1_u8));
    assert_eq!(v.wywy(), u8vec4(4_u8, 2_u8, 4_u8, 2_u8));
    assert_eq!(v.wywz(), u8vec4(4_u8, 2_u8, 4_u8, 3_u8));
    assert_eq!(v.wyww(), u8vec4(4_u8, 2_u8, 4_u8, 4_u8));
    assert_eq!(v.wzxx(), u8vec4(4_u8, 3_u8, 1_u8, 1_u8));
    assert_eq!(v.wzxy(), u8vec4(4_u8, 3_u8, 1_u8, 2_u8));
    assert_eq!(v.wzxz(), u8vec4(4_u8, 3_u8, 1_u8, 3_u8));
    assert_eq!(v.wzxw(), u8vec4(4_u8, 3_u8, 1_u8, 4_u8));
    assert_eq!(v.wzyx(), u8vec4(4_u8, 3_u8, 2_u8, 1_u8));
    assert_eq!(v.wzyy(), u8vec4(4_u8, 3_u8, 2_u8, 2_u8));
    assert_eq!(v.wzyz(), u8vec4(4_u8, 3_u8, 2_u8, 3_u8));
    assert_eq!(v.wzyw(), u8vec4(4_u8, 3_u8, 2_u8, 4_u8));
    assert_eq!(v.wzzx(), u8vec4(4_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.wzzy(), u8vec4(4_u8, 3_u8, 3_u8, 2_u8));
    assert_eq!(v.wzzz(), u8vec4(4_u8, 3_u8, 3_u8, 3_u8));
    assert_eq!(v.wzzw(), u8vec4(4_u8, 3_u8, 3_u8, 4_u8));
    assert_eq!(v.wzwx(), u8vec4(4_u8, 3_u8, 4_u8, 1_u8));
    assert_eq!(v.wzwy(), u8vec4(4_u8, 3_u8, 4_u8, 2_u8));
    assert_eq!(v.wzwz(), u8vec4(4_u8, 3_u8, 4_u8, 3_u8));
    assert_eq!(v.wzww(), u8vec4(4_u8, 3_u8, 4_u8, 4_u8));
    assert_eq!(v.wwxx(), u8vec4(4_u8, 4_u8, 1_u8, 1_u8));
    assert_eq!(v.wwxy(), u8vec4(4_u8, 4_u8, 1_u8, 2_u8));
    assert_eq!(v.wwxz(), u8vec4(4_u8, 4_u8, 1_u8, 3_u8));
    assert_eq!(v.wwxw(), u8vec4(4_u8, 4_u8, 1_u8, 4_u8));
    assert_eq!(v.wwyx(), u8vec4(4_u8, 4_u8, 2_u8, 1_u8));
    assert_eq!(v.wwyy(), u8vec4(4_u8, 4_u8, 2_u8, 2_u8));
    assert_eq!(v.wwyz(), u8vec4(4_u8, 4_u8, 2_u8, 3_u8));
    assert_eq!(v.wwyw(), u8vec4(4_u8, 4_u8, 2_u8, 4_u8));
    assert_eq!(v.wwzx(), u8vec4(4_u8, 4_u8, 3_u8, 1_u8));
    assert_eq!(v.wwzy(), u8vec4(4_u8, 4_u8, 3_u8, 2_u8));
    assert_eq!(v.wwzz(), u8vec4(4_u8, 4_u8, 3_u8, 3_u8));
    assert_eq!(v.wwzw(), u8vec4(4_u8, 4_u8, 3_u8, 4_u8));
    assert_eq!(v.wwwx(), u8vec4(4_u8, 4_u8, 4_u8, 1_u8));
    assert_eq!(v.wwwy(), u8vec4(4_u8, 4_u8, 4_u8, 2_u8));
    assert_eq!(v.wwwz(), u8vec4(4_u8, 4_u8, 4_u8, 3_u8));
    assert_eq!(v.wwww(), u8vec4(4_u8, 4_u8, 4_u8, 4_u8));
    assert_eq!(v.xxx(), u8vec3(1_u8, 1_u8, 1_u8));
    assert_eq!(v.xxy(), u8vec3(1_u8, 1_u8, 2_u8));
    assert_eq!(v.xxz(), u8vec3(1_u8, 1_u8, 3_u8));
    assert_eq!(v.xxw(), u8vec3(1_u8, 1_u8, 4_u8));
    assert_eq!(v.xyx(), u8vec3(1_u8, 2_u8, 1_u8));
    assert_eq!(v.xyy(), u8vec3(1_u8, 2_u8, 2_u8));
    assert_eq!(v.xyz(), u8vec3(1_u8, 2_u8, 3_u8));
    assert_eq!(v.xyw(), u8vec3(1_u8, 2_u8, 4_u8));
    assert_eq!(v.xzx(), u8vec3(1_u8, 3_u8, 1_u8));
    assert_eq!(v.xzy(), u8vec3(1_u8, 3_u8, 2_u8));
    assert_eq!(v.xzz(), u8vec3(1_u8, 3_u8, 3_u8));
    assert_eq!(v.xzw(), u8vec3(1_u8, 3_u8, 4_u8));
    assert_eq!(v.xwx(), u8vec3(1_u8, 4_u8, 1_u8));
    assert_eq!(v.xwy(), u8vec3(1_u8, 4_u8, 2_u8));
    assert_eq!(v.xwz(), u8vec3(1_u8, 4_u8, 3_u8));
    assert_eq!(v.xww(), u8vec3(1_u8, 4_u8, 4_u8));
    assert_eq!(v.yxx(), u8vec3(2_u8, 1_u8, 1_u8));
    assert_eq!(v.yxy(), u8vec3(2_u8, 1_u8, 2_u8));
    assert_eq!(v.yxz(), u8vec3(2_u8, 1_u8, 3_u8));
    assert_eq!(v.yxw(), u8vec3(2_u8, 1_u8, 4_u8));
    assert_eq!(v.yyx(), u8vec3(2_u8, 2_u8, 1_u8));
    assert_eq!(v.yyy(), u8vec3(2_u8, 2_u8, 2_u8));
    assert_eq!(v.yyz(), u8vec3(2_u8, 2_u8, 3_u8));
    assert_eq!(v.yyw(), u8vec3(2_u8, 2_u8, 4_u8));
    assert_eq!(v.yzx(), u8vec3(2_u8, 3_u8, 1_u8));
    assert_eq!(v.yzy(), u8vec3(2_u8, 3_u8, 2_u8));
    assert_eq!(v.yzz(), u8vec3(2_u8, 3_u8, 3_u8));
    assert_eq!(v.yzw(), u8vec3(2_u8, 3_u8, 4_u8));
    assert_eq!(v.ywx(), u8vec3(2_u8, 4_u8, 1_u8));
    assert_eq!(v.ywy(), u8vec3(2_u8, 4_u8, 2_u8));
    assert_eq!(v.ywz(), u8vec3(2_u8, 4_u8, 3_u8));
    assert_eq!(v.yww(), u8vec3(2_u8, 4_u8, 4_u8));
    assert_eq!(v.zxx(), u8vec3(3_u8, 1_u8, 1_u8));
    assert_eq!(v.zxy(), u8vec3(3_u8, 1_u8, 2_u8));
    assert_eq!(v.zxz(), u8vec3(3_u8, 1_u8, 3_u8));
    assert_eq!(v.zxw(), u8vec3(3_u8, 1_u8, 4_u8));
    assert_eq!(v.zyx(), u8vec3(3_u8, 2_u8, 1_u8));
    assert_eq!(v.zyy(), u8vec3(3_u8, 2_u8, 2_u8));
    assert_eq!(v.zyz(), u8vec3(3_u8, 2_u8, 3_u8));
    assert_eq!(v.zyw(), u8vec3(3_u8, 2_u8, 4_u8));
    assert_eq!(v.zzx(), u8vec3(3_u8, 3_u8, 1_u8));
    assert_eq!(v.zzy(), u8vec3(3_u8, 3_u8, 2_u8));
    assert_eq!(v.zzz(), u8vec3(3_u8, 3_u8, 3_u8));
    assert_eq!(v.zzw(), u8vec3(3_u8, 3_u8, 4_u8));
    assert_eq!(v.zwx(), u8vec3(3_u8, 4_u8, 1_u8));
    assert_eq!(v.zwy(), u8vec3(3_u8, 4_u8, 2_u8));
    assert_eq!(v.zwz(), u8vec3(3_u8, 4_u8, 3_u8));
    assert_eq!(v.zww(), u8vec3(3_u8, 4_u8, 4_u8));
    assert_eq!(v.wxx(), u8vec3(4_u8, 1_u8, 1_u8));
    assert_eq!(v.wxy(), u8vec3(4_u8, 1_u8, 2_u8));
    assert_eq!(v.wxz(), u8vec3(4_u8, 1_u8, 3_u8));
    assert_eq!(v.wxw(), u8vec3(4_u8, 1_u8, 4_u8));
    assert_eq!(v.wyx(), u8vec3(4_u8, 2_u8, 1_u8));
    assert_eq!(v.wyy(), u8vec3(4_u8, 2_u8, 2_u8));
    assert_eq!(v.wyz(), u8vec3(4_u8, 2_u8, 3_u8));
    assert_eq!(v.wyw(), u8vec3(4_u8, 2_u8, 4_u8));
    assert_eq!(v.wzx(), u8vec3(4_u8, 3_u8, 1_u8));
    assert_eq!(v.wzy(), u8vec3(4_u8, 3_u8, 2_u8));
    assert_eq!(v.wzz(), u8vec3(4_u8, 3_u8, 3_u8));
    assert_eq!(v.wzw(), u8vec3(4_u8, 3_u8, 4_u8));
    assert_eq!(v.wwx(), u8vec3(4_u8, 4_u8, 1_u8));
    assert_eq!(v.wwy(), u8vec3(4_u8, 4_u8, 2_u8));
    assert_eq!(v.wwz(), u8vec3(4_u8, 4_u8, 3_u8));
    assert_eq!(v.www(), u8vec3(4_u8, 4_u8, 4_u8));
    assert_eq!(v.with_xyz(u8vec3(2_u8, 3_u8, 4_u8)), u8vec4(2_u8, 3_u8, 4_u8, 4_u8));
    assert_eq!(v.with_xyw(u8vec3(2_u8, 3_u8, 1_u8)), u8vec4(2_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.with_xzy(u8vec3(2_u8, 4_u8, 3_u8)), u8vec4(2_u8, 3_u8, 4_u8, 4_u8));
    assert_eq!(v.with_xzw(u8vec3(2_u8, 4_u8, 1_u8)), u8vec4(2_u8, 2_u8, 4_u8, 1_u8));
    assert_eq!(v.with_xwy(u8vec3(2_u8, 1_u8, 3_u8)), u8vec4(2_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.with_xwz(u8vec3(2_u8, 1_u8, 4_u8)), u8vec4(2_u8, 2_u8, 4_u8, 1_u8));
    assert_eq!(v.with_yxz(u8vec3(3_u8, 2_u8, 4_u8)), u8vec4(2_u8, 3_u8, 4_u8, 4_u8));
    assert_eq!(v.with_yxw(u8vec3(3_u8, 2_u8, 1_u8)), u8vec4(2_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.with_yzx(u8vec3(3_u8, 4_u8, 2_u8)), u8vec4(2_u8, 3_u8, 4_u8, 4_u8));
    assert_eq!(v.with_yzw(u8vec3(3_u8, 4_u8, 1_u8)), u8vec4(1_u8, 3_u8, 4_u8, 1_u8));
    assert_eq!(v.with_ywx(u8vec3(3_u8, 1_u8, 2_u8)), u8vec4(2_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.with_ywz(u8vec3(3_u8, 1_u8, 4_u8)), u8vec4(1_u8, 3_u8, 4_u8, 1_u8));
    assert_eq!(v.with_zxy(u8vec3(4_u8, 2_u8, 3_u8)), u8vec4(2_u8, 3_u8, 4_u8, 4_u8));
    assert_eq!(v.with_zxw(u8vec3(4_u8, 2_u8, 1_u8)), u8vec4(2_u8, 2_u8, 4_u8, 1_u8));
    assert_eq!(v.with_zyx(u8vec3(4_u8, 3_u8, 2_u8)), u8vec4(2_u8, 3_u8, 4_u8, 4_u8));
    assert_eq!(v.with_zyw(u8vec3(4_u8, 3_u8, 1_u8)), u8vec4(1_u8, 3_u8, 4_u8, 1_u8));
    assert_eq!(v.with_zwx(u8vec3(4_u8, 1_u8, 2_u8)), u8vec4(2_u8, 2_u8, 4_u8, 1_u8));
    assert_eq!(v.with_zwy(u8vec3(4_u8, 1_u8, 3_u8)), u8vec4(1_u8, 3_u8, 4_u8, 1_u8));
    assert_eq!(v.with_wxy(u8vec3(1_u8, 2_u8, 3_u8)), u8vec4(2_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.with_wxz(u8vec3(1_u8, 2_u8, 4_u8)), u8vec4(2_u8, 2_u8, 4_u8, 1_u8));
    assert_eq!(v.with_wyx(u8vec3(1_u8, 3_u8, 2_u8)), u8vec4(2_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.with_wyz(u8vec3(1_u8, 3_u8, 4_u8)), u8vec4(1_u8, 3_u8, 4_u8, 1_u8));
    assert_eq!(v.with_wzx(u8vec3(1_u8, 4_u8, 2_u8)), u8vec4(2_u8, 2_u8, 4_u8, 1_u8));
    assert_eq!(v.with_wzy(u8vec3(1_u8, 4_u8, 3_u8)), u8vec4(1_u8, 3_u8, 4_u8, 1_u8));
    assert_eq!(v.xx(), u8vec2(1_u8, 1_u8));
    assert_eq!(v.xy(), u8vec2(1_u8, 2_u8));
    assert_eq!(v.xz(), u8vec2(1_u8, 3_u8));
    assert_eq!(v.xw(), u8vec2(1_u8, 4_u8));
    assert_eq!(v.yx(), u8vec2(2_u8, 1_u8));
    assert_eq!(v.yy(), u8vec2(2_u8, 2_u8));
    assert_eq!(v.yz(), u8vec2(2_u8, 3_u8));
    assert_eq!(v.yw(), u8vec2(2_u8, 4_u8));
    assert_eq!(v.zx(), u8vec2(3_u8, 1_u8));
    assert_eq!(v.zy(), u8vec2(3_u8, 2_u8));
    assert_eq!(v.zz(), u8vec2(3_u8, 3_u8));
    assert_eq!(v.zw(), u8vec2(3_u8, 4_u8));
    assert_eq!(v.wx(), u8vec2(4_u8, 1_u8));
    assert_eq!(v.wy(), u8vec2(4_u8, 2_u8));
    assert_eq!(v.wz(), u8vec2(4_u8, 3_u8));
    assert_eq!(v.ww(), u8vec2(4_u8, 4_u8));
    assert_eq!(v.with_xy(u8vec2(2_u8, 3_u8)), u8vec4(2_u8, 3_u8, 3_u8, 4_u8));
    assert_eq!(v.with_xz(u8vec2(2_u8, 4_u8)), u8vec4(2_u8, 2_u8, 4_u8, 4_u8));
    assert_eq!(v.with_xw(u8vec2(2_u8, 1_u8)), u8vec4(2_u8, 2_u8, 3_u8, 1_u8));
    assert_eq!(v.with_yx(u8vec2(3_u8, 2_u8)), u8vec4(2_u8, 3_u8, 3_u8, 4_u8));
    assert_eq!(v.with_yz(u8vec2(3_u8, 4_u8)), u8vec4(1_u8, 3_u8, 4_u8, 4_u8));
    assert_eq!(v.with_yw(u8vec2(3_u8, 1_u8)), u8vec4(1_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.with_zx(u8vec2(4_u8, 2_u8)), u8vec4(2_u8, 2_u8, 4_u8, 4_u8));
    assert_eq!(v.with_zy(u8vec2(4_u8, 3_u8)), u8vec4(1_u8, 3_u8, 4_u8, 4_u8));
    assert_eq!(v.with_zw(u8vec2(4_u8, 1_u8)), u8vec4(1_u8, 2_u8, 4_u8, 1_u8));
    assert_eq!(v.with_wx(u8vec2(1_u8, 2_u8)), u8vec4(2_u8, 2_u8, 3_u8, 1_u8));
    assert_eq!(v.with_wy(u8vec2(1_u8, 3_u8)), u8vec4(1_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.with_wz(u8vec2(1_u8, 4_u8)), u8vec4(1_u8, 2_u8, 4_u8, 1_u8));
});

glam_test!(test_u8vec3_swizzles, {
    let v = u8vec3(1_u8, 2_u8, 3_u8);
    assert_eq!(v, v.xyz());
    assert_eq!(v.xxxx(), u8vec4(1_u8, 1_u8, 1_u8, 1_u8));
    assert_eq!(v.xxxy(), u8vec4(1_u8, 1_u8, 1_u8, 2_u8));
    assert_eq!(v.xxxz(), u8vec4(1_u8, 1_u8, 1_u8, 3_u8));
    assert_eq!(v.xxyx(), u8vec4(1_u8, 1_u8, 2_u8, 1_u8));
    assert_eq!(v.xxyy(), u8vec4(1_u8, 1_u8, 2_u8, 2_u8));
    assert_eq!(v.xxyz(), u8vec4(1_u8, 1_u8, 2_u8, 3_u8));
    assert_eq!(v.xxzx(), u8vec4(1_u8, 1_u8, 3_u8, 1_u8));
    assert_eq!(v.xxzy(), u8vec4(1_u8, 1_u8, 3_u8, 2_u8));
    assert_eq!(v.xxzz(), u8vec4(1_u8, 1_u8, 3_u8, 3_u8));
    assert_eq!(v.xyxx(), u8vec4(1_u8, 2_u8, 1_u8, 1_u8));
    assert_eq!(v.xyxy(), u8vec4(1_u8, 2_u8, 1_u8, 2_u8));
    assert_eq!(v.xyxz(), u8vec4(1_u8, 2_u8, 1_u8, 3_u8));
    assert_eq!(v.xyyx(), u8vec4(1_u8, 2_u8, 2_u8, 1_u8));
    assert_eq!(v.xyyy(), u8vec4(1_u8, 2_u8, 2_u8, 2_u8));
    assert_eq!(v.xyyz(), u8vec4(1_u8, 2_u8, 2_u8, 3_u8));
    assert_eq!(v.xyzx(), u8vec4(1_u8, 2_u8, 3_u8, 1_u8));
    assert_eq!(v.xyzy(), u8vec4(1_u8, 2_u8, 3_u8, 2_u8));
    assert_eq!(v.xyzz(), u8vec4(1_u8, 2_u8, 3_u8, 3_u8));
    assert_eq!(v.xzxx(), u8vec4(1_u8, 3_u8, 1_u8, 1_u8));
    assert_eq!(v.xzxy(), u8vec4(1_u8, 3_u8, 1_u8, 2_u8));
    assert_eq!(v.xzxz(), u8vec4(1_u8, 3_u8, 1_u8, 3_u8));
    assert_eq!(v.xzyx(), u8vec4(1_u8, 3_u8, 2_u8, 1_u8));
    assert_eq!(v.xzyy(), u8vec4(1_u8, 3_u8, 2_u8, 2_u8));
    assert_eq!(v.xzyz(), u8vec4(1_u8, 3_u8, 2_u8, 3_u8));
    assert_eq!(v.xzzx(), u8vec4(1_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.xzzy(), u8vec4(1_u8, 3_u8, 3_u8, 2_u8));
    assert_eq!(v.xzzz(), u8vec4(1_u8, 3_u8, 3_u8, 3_u8));
    assert_eq!(v.yxxx(), u8vec4(2_u8, 1_u8, 1_u8, 1_u8));
    assert_eq!(v.yxxy(), u8vec4(2_u8, 1_u8, 1_u8, 2_u8));
    assert_eq!(v.yxxz(), u8vec4(2_u8, 1_u8, 1_u8, 3_u8));
    assert_eq!(v.yxyx(), u8vec4(2_u8, 1_u8, 2_u8, 1_u8));
    assert_eq!(v.yxyy(), u8vec4(2_u8, 1_u8, 2_u8, 2_u8));
    assert_eq!(v.yxyz(), u8vec4(2_u8, 1_u8, 2_u8, 3_u8));
    assert_eq!(v.yxzx(), u8vec4(2_u8, 1_u8, 3_u8, 1_u8));
    assert_eq!(v.yxzy(), u8vec4(2_u8, 1_u8, 3_u8, 2_u8));
    assert_eq!(v.yxzz(), u8vec4(2_u8, 1_u8, 3_u8, 3_u8));
    assert_eq!(v.yyxx(), u8vec4(2_u8, 2_u8, 1_u8, 1_u8));
    assert_eq!(v.yyxy(), u8vec4(2_u8, 2_u8, 1_u8, 2_u8));
    assert_eq!(v.yyxz(), u8vec4(2_u8, 2_u8, 1_u8, 3_u8));
    assert_eq!(v.yyyx(), u8vec4(2_u8, 2_u8, 2_u8, 1_u8));
    assert_eq!(v.yyyy(), u8vec4(2_u8, 2_u8, 2_u8, 2_u8));
    assert_eq!(v.yyyz(), u8vec4(2_u8, 2_u8, 2_u8, 3_u8));
    assert_eq!(v.yyzx(), u8vec4(2_u8, 2_u8, 3_u8, 1_u8));
    assert_eq!(v.yyzy(), u8vec4(2_u8, 2_u8, 3_u8, 2_u8));
    assert_eq!(v.yyzz(), u8vec4(2_u8, 2_u8, 3_u8, 3_u8));
    assert_eq!(v.yzxx(), u8vec4(2_u8, 3_u8, 1_u8, 1_u8));
    assert_eq!(v.yzxy(), u8vec4(2_u8, 3_u8, 1_u8, 2_u8));
    assert_eq!(v.yzxz(), u8vec4(2_u8, 3_u8, 1_u8, 3_u8));
    assert_eq!(v.yzyx(), u8vec4(2_u8, 3_u8, 2_u8, 1_u8));
    assert_eq!(v.yzyy(), u8vec4(2_u8, 3_u8, 2_u8, 2_u8));
    assert_eq!(v.yzyz(), u8vec4(2_u8, 3_u8, 2_u8, 3_u8));
    assert_eq!(v.yzzx(), u8vec4(2_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.yzzy(), u8vec4(2_u8, 3_u8, 3_u8, 2_u8));
    assert_eq!(v.yzzz(), u8vec4(2_u8, 3_u8, 3_u8, 3_u8));
    assert_eq!(v.zxxx(), u8vec4(3_u8, 1_u8, 1_u8, 1_u8));
    assert_eq!(v.zxxy(), u8vec4(3_u8, 1_u8, 1_u8, 2_u8));
    assert_eq!(v.zxxz(), u8vec4(3_u8, 1_u8, 1_u8, 3_u8));
    assert_eq!(v.zxyx(), u8vec4(3_u8, 1_u8, 2_u8, 1_u8));
    assert_eq!(v.zxyy(), u8vec4(3_u8, 1_u8, 2_u8, 2_u8));
    assert_eq!(v.zxyz(), u8vec4(3_u8, 1_u8, 2_u8, 3_u8));
    assert_eq!(v.zxzx(), u8vec4(3_u8, 1_u8, 3_u8, 1_u8));
    assert_eq!(v.zxzy(), u8vec4(3_u8, 1_u8, 3_u8, 2_u8));
    assert_eq!(v.zxzz(), u8vec4(3_u8, 1_u8, 3_u8, 3_u8));
    assert_eq!(v.zyxx(), u8vec4(3_u8, 2_u8, 1_u8, 1_u8));
    assert_eq!(v.zyxy(), u8vec4(3_u8, 2_u8, 1_u8, 2_u8));
    assert_eq!(v.zyxz(), u8vec4(3_u8, 2_u8, 1_u8, 3_u8));
    assert_eq!(v.zyyx(), u8vec4(3_u8, 2_u8, 2_u8, 1_u8));
    assert_eq!(v.zyyy(), u8vec4(3_u8, 2_u8, 2_u8, 2_u8));
    assert_eq!(v.zyyz(), u8vec4(3_u8, 2_u8, 2_u8, 3_u8));
    assert_eq!(v.zyzx(), u8vec4(3_u8, 2_u8, 3_u8, 1_u8));
    assert_eq!(v.zyzy(), u8vec4(3_u8, 2_u8, 3_u8, 2_u8));
    assert_eq!(v.zyzz(), u8vec4(3_u8, 2_u8, 3_u8, 3_u8));
    assert_eq!(v.zzxx(), u8vec4(3_u8, 3_u8, 1_u8, 1_u8));
    assert_eq!(v.zzxy(), u8vec4(3_u8, 3_u8, 1_u8, 2_u8));
    assert_eq!(v.zzxz(), u8vec4(3_u8, 3_u8, 1_u8, 3_u8));
    assert_eq!(v.zzyx(), u8vec4(3_u8, 3_u8, 2_u8, 1_u8));
    assert_eq!(v.zzyy(), u8vec4(3_u8, 3_u8, 2_u8, 2_u8));
    assert_eq!(v.zzyz(), u8vec4(3_u8, 3_u8, 2_u8, 3_u8));
    assert_eq!(v.zzzx(), u8vec4(3_u8, 3_u8, 3_u8, 1_u8));
    assert_eq!(v.zzzy(), u8vec4(3_u8, 3_u8, 3_u8, 2_u8));
    assert_eq!(v.zzzz(), u8vec4(3_u8, 3_u8, 3_u8, 3_u8));
    assert_eq!(v.xxx(), u8vec3(1_u8, 1_u8, 1_u8));
    assert_eq!(v.xxy(), u8vec3(1_u8, 1_u8, 2_u8));
    assert_eq!(v.xxz(), u8vec3(1_u8, 1_u8, 3_u8));
    assert_eq!(v.xyx(), u8vec3(1_u8, 2_u8, 1_u8));
    assert_eq!(v.xyy(), u8vec3(1_u8, 2_u8, 2_u8));
    assert_eq!(v.xzx(), u8vec3(1_u8, 3_u8, 1_u8));
    assert_eq!(v.xzy(), u8vec3(1_u8, 3_u8, 2_u8));
    assert_eq!(v.xzz(), u8vec3(1_u8, 3_u8, 3_u8));
    assert_eq!(v.yxx(), u8vec3(2_u8, 1_u8, 1_u8));
    assert_eq!(v.yxy(), u8vec3(2_u8, 1_u8, 2_u8));
    assert_eq!(v.yxz(), u8vec3(2_u8, 1_u8, 3_u8));
    assert_eq!(v.yyx(), u8vec3(2_u8, 2_u8, 1_u8));
    assert_eq!(v.yyy(), u8vec3(2_u8, 2_u8, 2_u8));
    assert_eq!(v.yyz(), u8vec3(2_u8, 2_u8, 3_u8));
    assert_eq!(v.yzx(), u8vec3(2_u8, 3_u8, 1_u8));
    assert_eq!(v.yzy(), u8vec3(2_u8, 3_u8, 2_u8));
    assert_eq!(v.yzz(), u8vec3(2_u8, 3_u8, 3_u8));
    assert_eq!(v.zxx(), u8vec3(3_u8, 1_u8, 1_u8));
    assert_eq!(v.zxy(), u8vec3(3_u8, 1_u8, 2_u8));
    assert_eq!(v.zxz(), u8vec3(3_u8, 1_u8, 3_u8));
    assert_eq!(v.zyx(), u8vec3(3_u8, 2_u8, 1_u8));
    assert_eq!(v.zyy(), u8vec3(3_u8, 2_u8, 2_u8));
    assert_eq!(v.zyz(), u8vec3(3_u8, 2_u8, 3_u8));
    assert_eq!(v.zzx(), u8vec3(3_u8, 3_u8, 1_u8));
    assert_eq!(v.zzy(), u8vec3(3_u8, 3_u8, 2_u8));
    assert_eq!(v.zzz(), u8vec3(3_u8, 3_u8, 3_u8));
    assert_eq!(v.xx(), u8vec2(1_u8, 1_u8));
    assert_eq!(v.xy(), u8vec2(1_u8, 2_u8));
    assert_eq!(v.xz(), u8vec2(1_u8, 3_u8));
    assert_eq!(v.yx(), u8vec2(2_u8, 1_u8));
    assert_eq!(v.yy(), u8vec2(2_u8, 2_u8));
    assert_eq!(v.yz(), u8vec2(2_u8, 3_u8));
    assert_eq!(v.zx(), u8vec2(3_u8, 1_u8));
    assert_eq!(v.zy(), u8vec2(3_u8, 2_u8));
    assert_eq!(v.zz(), u8vec2(3_u8, 3_u8));
    assert_eq!(v.with_xy(u8vec2(2_u8, 3_u8)), u8vec3(2_u8, 3_u8, 3_u8));
    assert_eq!(v.with_xz(u8vec2(2_u8, 1_u8)), u8vec3(2_u8, 2_u8, 1_u8));
    assert_eq!(v.with_yx(u8vec2(3_u8, 2_u8)), u8vec3(2_u8, 3_u8, 3_u8));
    assert_eq!(v.with_yz(u8vec2(3_u8, 1_u8)), u8vec3(1_u8, 3_u8, 1_u8));
    assert_eq!(v.with_zx(u8vec2(1_u8, 2_u8)), u8vec3(2_u8, 2_u8, 1_u8));
    assert_eq!(v.with_zy(u8vec2(1_u8, 3_u8)), u8vec3(1_u8, 3_u8, 1_u8));
});

glam_test!(test_u8vec2_swizzles, {
    let v = u8vec2(1_u8, 2_u8);
    assert_eq!(v, v.xy());
    assert_eq!(v.xxxx(), u8vec4(1_u8, 1_u8, 1_u8, 1_u8));
    assert_eq!(v.xxxy(), u8vec4(1_u8, 1_u8, 1_u8, 2_u8));
    assert_eq!(v.xxyx(), u8vec4(1_u8, 1_u8, 2_u8, 1_u8));
    assert_eq!(v.xxyy(), u8vec4(1_u8, 1_u8, 2_u8, 2_u8));
    assert_eq!(v.xyxx(), u8vec4(1_u8, 2_u8, 1_u8, 1_u8));
    assert_eq!(v.xyxy(), u8vec4(1_u8, 2_u8, 1_u8, 2_u8));
    assert_eq!(v.xyyx(), u8vec4(1_u8, 2_u8, 2_u8, 1_u8));
    assert_eq!(v.xyyy(), u8vec4(1_u8, 2_u8, 2_u8, 2_u8));
    assert_eq!(v.yxxx(), u8vec4(2_u8, 1_u8, 1_u8, 1_u8));
    assert_eq!(v.yxxy(), u8vec4(2_u8, 1_u8, 1_u8, 2_u8));
    assert_eq!(v.yxyx(), u8vec4(2_u8, 1_u8, 2_u8, 1_u8));
    assert_eq!(v.yxyy(), u8vec4(2_u8, 1_u8, 2_u8, 2_u8));
    assert_eq!(v.yyxx(), u8vec4(2_u8, 2_u8, 1_u8, 1_u8));
    assert_eq!(v.yyxy(), u8vec4(2_u8, 2_u8, 1_u8, 2_u8));
    assert_eq!(v.yyyx(), u8vec4(2_u8, 2_u8, 2_u8, 1_u8));
    assert_eq!(v.yyyy(), u8vec4(2_u8, 2_u8, 2_u8, 2_u8));
    assert_eq!(v.xxx(), u8vec3(1_u8, 1_u8, 1_u8));
    assert_eq!(v.xxy(), u8vec3(1_u8, 1_u8, 2_u8));
    assert_eq!(v.xyx(), u8vec3(1_u8, 2_u8, 1_u8));
    assert_eq!(v.xyy(), u8vec3(1_u8, 2_u8, 2_u8));
    assert_eq!(v.yxx(), u8vec3(2_u8, 1_u8, 1_u8));
    assert_eq!(v.yxy(), u8vec3(2_u8, 1_u8, 2_u8));
    assert_eq!(v.yyx(), u8vec3(2_u8, 2_u8, 1_u8));
    assert_eq!(v.yyy(), u8vec3(2_u8, 2_u8, 2_u8));
    assert_eq!(v.xx(), u8vec2(1_u8, 1_u8));
    assert_eq!(v.yx(), u8vec2(2_u8, 1_u8));
    assert_eq!(v.yy(), u8vec2(2_u8, 2_u8));
});
