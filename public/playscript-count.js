// Line and character counting function
//
//
// ## 入力データ
//
// <div class="speech">
//   <h5 id="D0">...</h5>
//   <p>...</p>
// </div>
//
// `div.speech > p`の中にはテキストあるいはト書きのspan要素が入っている。
// これらの文字数を行数を数える。このとき、ruby要素に関してはルビは無視する。
// 行数は行の高さでp要素の高さを割って切り上げる。
// 一番上の行は行の高さいっぱいまで使わない。
//
/// Returns the inner paragraph of `div.speech`
function stripSpeech(sp) {
	const p = sp.querySelector('p');

	return p;
}

/// Counts characters in an inner paragraph of `div.speech`
function countCharacters(p) {
	let count = 0;
	for ( node of p.childNodes ) {
		count += countCharsInSpan(node);
	}

	return count;
}

function countCharsInSpan(span) {
	let count = 0;

	for ( node of span.childNodes ) {
		if ( node instanceof Text ) {
			count += node.length;
		} else if ( node.tagName === "RUBY" ) {
			count += countCharsInRuby(node);
		}
	}

	return count;
}

/// Counts the number of base characters in the given RUBY element.
function countCharsInRuby(ruby) {
	let count = 0;

	for ( node of ruby.childNodes ) {
		if ( node instanceof Text ) {
			count += node.length;
		}
	}

	return count;
}

const holders = document.querySelectorAll('div#playscript-count');

const holder = holders.item(holders.length - 1);

if ( holder ) {
	const script = document.querySelectorAll('div.speech');

	let characters = 0;
	let lines = 0;

	for ( speech of script ) {
		const p = stripSpeech(speech);

		characters += countCharacters(p);

		const pHeight = p.offsetHeight;

		const style = window.getComputedStyle(p);
		const lineHeight = parseFloat(style.getPropertyValue('line-height'));

		lines += Math.ceil(pHeight / lineHeight);
	}

	holder.innerText = lines + '行, ' + characters + '字';
}
