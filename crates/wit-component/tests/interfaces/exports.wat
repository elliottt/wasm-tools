(component
  (type (;0;)
    (component
      (type (;0;)
        (instance
          (type (;0;) (record (field "a" u32)))
          (export (;1;) "my-struct" (type (eq 0)))
          (type (;2;) (func (param "a" 1) (result string)))
          (export (;0;) "my-function" (func (type 2)))
        )
      )
      (export (;0;) (interface "foo:foo/foo") (instance (type 0)))
    )
  )
  (export (;1;) "foo" (type 0))
  (type (;2;)
    (component
      (type (;0;)
        (component
          (type (;0;)
            (instance
              (type (;0;) (record (field "a" u32)))
              (export (;1;) "my-struct" (type (eq 0)))
              (type (;2;) (func (param "a" 1) (result string)))
              (export (;0;) "my-function" (func (type 2)))
            )
          )
          (export (;0;) (interface "foo:foo/foo") (instance (type 0)))
        )
      )
      (export (;1;) (interface "foo:foo/export-foo") (type (eq 0)))
    )
  )
  (export (;3;) "export-foo" (type 2))
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)