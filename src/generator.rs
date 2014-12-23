use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::ptr::P;

use block::Block;

pub fn generate(cx: &mut ExtCtxt, block: &Block) -> P<ast::Item> {
    match *block {
        Block::Describe {ref name, ref blocks, ..} => {
            let name = cx.ident_of(name.as_slice());
            let items = blocks.iter().map(|block| {
                generate(cx, block)
            }).collect::<Vec<_>>();

            quote_item!(cx,
                mod $name {
                    $items
                }
            ).expect("failed to generate `ast::Item` from `Block::Describe`!")
        },

        Block::It {ref name, ref block} => {
            let name = cx.ident_of(name.as_slice());
            quote_item!(cx,
                #[test]
                fn $name() {
                    $block
                }
            ).expect("failed to generate `ast::Item` from `Block::It`!")
        }
    }
}
