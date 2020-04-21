::mini_paste::item! {
    fn [< ma in >] ()
    {
        _main()
    }
}

fn _main ()
{
    ::mini_paste::item! {
        #[allow(nonstandard_style)]
        const [< i t >] : () = ();
    }
    ::mini_paste::expr!({
        [< print ln >] ! ("Hello, World!");
        [< i t >]
    })
}
