(component
  (type (;0;)
    (component
      (type (;0;)
        (instance
          (type (;0;) u8)
          (export (;1;) "t1" (type (eq 0)))
        )
      )
      (export (;0;) (interface "foo:foo/shared1") (instance (type 0)))
    )
  )
  (export (;1;) "shared1" (type 0))
  (type (;2;)
    (component
      (type (;0;)
        (instance
          (type (;0;) u8)
          (export (;1;) "t2" (type (eq 0)))
        )
      )
      (export (;0;) (interface "foo:foo/shared2") (instance (type 0)))
    )
  )
  (export (;3;) "shared2" (type 2))
  (type (;4;)
    (component
      (type (;0;)
        (component
          (type (;0;)
            (instance
              (type (;0;) u8)
              (export (;1;) "t1" (type (eq 0)))
            )
          )
          (import (interface "foo:foo/shared1") (instance (;0;) (type 0)))
          (alias export 0 "t1" (type (;1;)))
          (type (;2;)
            (instance
              (alias outer 1 1 (type (;0;)))
              (export (;1;) "t1" (type (eq 0)))
            )
          )
          (import "foo" (instance (;1;) (type 2)))
          (type (;3;)
            (instance
              (type (;0;) u8)
              (export (;1;) "t2" (type (eq 0)))
            )
          )
          (import (interface "foo:foo/shared2") (instance (;2;) (type 3)))
          (alias export 2 "t2" (type (;4;)))
          (type (;5;)
            (instance
              (alias outer 1 4 (type (;0;)))
              (export (;1;) "t2" (type (eq 0)))
            )
          )
          (import "bar" (instance (;3;) (type 5)))
        )
      )
      (export (;1;) (interface "foo:foo/w1") (type (eq 0)))
    )
  )
  (export (;5;) "w1" (type 4))
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)