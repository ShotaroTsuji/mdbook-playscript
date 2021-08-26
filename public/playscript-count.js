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

const holder = document.getElementById('playscript-count');

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
