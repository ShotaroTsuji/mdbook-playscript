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

/// Gets a class name starts with `scene`
function getSceneClassName(elem) {
	for ( className of elem.classList.values() ) {
		if ( className.startsWith('scene') ) {
			return className;
		}
	}
}

function countCharactersInSpeeches(holderMap) {
	const script = document.querySelectorAll('div.speech');

	const lineCountMap = new Map();
	const charCountMap = new Map();

	let characters = 0;
	let lines = 0;

	for ( speech of script ) {
		const sceneName = getSceneClassName(speech);
		const p = stripSpeech(speech);

		const pChars = countCharacters(p);

		if ( charCountMap.has(sceneName) ) {
			const chars = charCountMap.get(sceneName);
			charCountMap.set(sceneName, chars + pChars);
		} else {
			charCountMap.set(sceneName, pChars);
		}

		const pHeight = p.offsetHeight;

		const style = window.getComputedStyle(p);
		const lineHeight = parseFloat(style.getPropertyValue('line-height'));

		const pLines = Math.ceil(pHeight / lineHeight);

		if ( lineCountMap.has(sceneName) ) {
			const lines = lineCountMap.get(sceneName);
			lineCountMap.set(sceneName, lines + pLines);
		} else {
			lineCountMap.set(sceneName, pLines);
		}
	}

	for ( pair of holderMap.entries() ) {
		const lines = lineCountMap.get(pair[0]);
		const characters = charCountMap.get(pair[0]);
		pair[1].innerText = lines + '行, ' + characters + '字';
	}
}

const holders = document.getElementsByClassName('mdplayscript-count');

// a mapping from scene class name to place holder element
const holderMap = new Map();

for ( holder of holders ) {
	for ( className of holder.classList.values() ) {
		if ( className.startsWith('scene') ) {
			holderMap.set(className, holder);
		}
	}
}

console.log(holderMap);

countCharactersInSpeeches(holderMap);
