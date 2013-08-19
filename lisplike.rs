extern mod std;
use std::hashmap::HashMap;
use sexpr;

// A very simple LISP-like language
// Globally scoped, no closures

/// Our value types
#[deriving(Clone)]
pub enum LispValue {
	List(~[LispValue]),
	Atom(~str),
	Str(~str),
	Num(float),
	Fn(~[~str], ~LispValue), // args, body
	BIF(~str, ~[~str], extern fn(~[~LispValue])->~LispValue) // built-in function (args, closure)
}

// XXX: this is ugly but it won't automatically derive Eq because of the extern fn
impl Eq for LispValue {
	fn eq(&self, other: &LispValue) -> bool {
		match (self.clone(), other.clone()) {
			(BIF(ref x, _, _), BIF(ref y, _, _)) if *x == *y => true,
			(Str(ref x), Str(ref y)) if *x == *y => true,
			(Num(ref x), Num(ref y)) if *x == *y => true,
			(Atom(ref x), Atom(ref y)) if *x == *y => true,
			(List(ref x), List(ref y)) if *x == *y => true,
			(Fn(ref x, ref x2), Fn(ref y, ref y2)) if *x == *y && *x2 == *y2 => true,
			_ => false
		}
	}
}

fn from_sexpr(sexpr: &sexpr::Value) -> ~LispValue {
	match *sexpr {
		sexpr::List(ref v) => ~List(v.map(|x| *from_sexpr(x))),
		sexpr::Num(v) => ~Num(v),
		sexpr::Str(ref v) => ~Str(v.clone()),
		sexpr::Atom(ref v) => ~Atom(v.clone())
	}
}

/// The type of the global symbol table (string to a value mapping).
type SymbolTable = HashMap<~str, ~LispValue>;

/// Creates a new symbol table and returns it
pub fn new_symt() -> SymbolTable {
	HashMap::new()
}

/// Binds a symbol in the symbol table. Replaces if it already exists.
pub fn bind(symt: &mut SymbolTable, name: ~str, value: ~LispValue) {
	symt.insert(name, value);
}

/// Look up a symbol in the symbol table. Fails if not found.
pub fn lookup(symt: &SymbolTable, name: ~str) -> ~LispValue {
	match symt.find(&name) {
		Some(v) => v.clone(),
		None => fail!("couldn't find symbol: %s", name)
	}
}

fn id_(v: ~[~LispValue]) -> ~LispValue { v[0] }

/// Initializes standard library functions
pub fn init_std(symt: &mut SymbolTable) {
	bind(symt, ~"id", ~BIF(~"id", ~[~"x"], id_));
}

fn apply(symt: &mut SymbolTable, f: ~LispValue, args: ~[~LispValue]) -> ~LispValue {
	fail!("stub: apply")
}

/// Evaluates an s-expression and returns a value.
pub fn eval(symt: &mut SymbolTable, input: sexpr::Value) -> ~LispValue {
	match input {
		sexpr::List(v) => {
			if(v.len() == 0) {
				fail!("eval given empty list")
			}

			// XXX: If we don't clone, the `match` partially moves v,
			// so we can't use it in the match arms.
			let v_ = v.clone();

			// evaluate a list as a function call
			match v[0] {
				sexpr::Atom(sym) => {
					let f = lookup(symt, sym);
					let args = v_.slice(1, v_.len());
					let xargs = args.map(|x| eval(symt, x.clone())); // eval'd args
					apply(symt, f, xargs)
				}
				_ => fail!("function calls take an atom"),
			}

		}
		_ => from_sexpr(&input) // return non-list values as they are
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use sexpr;
	use sexpr::from_str;

	fn read(input: &str) -> sexpr::Value {
		from_str(input).unwrap()
	}

	#[test]
	fn test_eval() {
		let mut symt = new_symt();
		init_std(&mut symt);
		assert_eq!(eval(&mut symt, read("123")), ~Num(123.0));
		assert_eq!(eval(&mut symt, read("(id 123)")), ~Num(123.0));
		// should fail: assert_eq!(eval(&mut symt, read("(1 2 3)")), ~List(~[Num(1.0), Num(2.0), Num(3.0)]));
	}
}