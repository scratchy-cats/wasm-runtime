(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add)

	(func $start
		i32.const 2
		i32.const 3
		call $add
		drop)

	(start $start)
)
