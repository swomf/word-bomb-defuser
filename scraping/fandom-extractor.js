/**
 * The purpose of this script is to extract words from the Fandom articles
 * for punctuation words special to the game.
 *
 * Thank you, Fandom wiki writers, for storing the words in a regular pattern.
 * 
 * WARNING: This script is NOT perfect. AUDIT YOUR LISTS BEFORE USING.
 * e.g. Jack(s)-in-the-box(es)
 * e.g. Long-winded(ly)(ness[es])
 * 
 * Errors to look for:
 * "ing" issues
 * ( [
 */

(() => {
  clear();
  let p = document.querySelector('.mw-parser-output').children;
  let words = [];
  let pushWord = (word) => {
    if (word.length < 1) return; // cannot be empty
    console.log('WORD: ' + word);
    words.push(word);
  };

  for (let i = 0; i < p.length; i++) {
    // Filter <p> out
    if (p[i].tagName !== 'P') continue;
    // Make sure the word is above 1 letter, in case it is a letter header
    if (p[i].textContent.length < 2) continue;
    // Make sure it doesn't have spaces, in case it is a title
    if (p[i].textContent.trim().includes(' ')) continue;

    let word = p[i].textContent.trim();
    console.log('RAW WORD: ' + word + ' ---------------------');
    // Log if the word is normal, and does not have a paren or anything special
    // this assumes that the word list is properly formatted
    if (!(word.includes('(') || word.includes('[') || word.includes('/'))) {
      pushWord(word);
      continue;
    }

    /* If the word has ( [ or / it is multiple words put into one
       var word = "About-face(d)/ing(s)"
       var word = "Apple-polish(ed)/(ing)/(er[s])/(es)"
       var word = "Aunt(s)-in-law"
       var word = "Big-city/ies"
       var word = "First-rate(ness[es])/(r[s]) "
       var word = "Full-time/(r[s]) "
       var word = "Jack-o'-lantern(s)"
       var word = "Johnny-come-lately/ies"
    we need to split this up
  */
    console.log('Special case: ' + word);

    // CASE 1: Inner-replacement (s)-in-law (s)-of-all-trades case
    if (word.match(/[a-zA-Z-]\(s\)[a-zA-Z-]/)) {
      console.log('Inner replacement detected');
      let sPartsBeforeAfter = word.split('(s)');
      // IMPERFECTION: Jacks-in-the-box(es) will be added
      pushWord(sPartsBeforeAfter[0] + sPartsBeforeAfter[1]);
      pushWord(sPartsBeforeAfter[0] + 's' + sPartsBeforeAfter[1]);
      continue;
    }

    // CASE 2: End-replacement y/ies case
    if (word.endsWith('y/ies')) {
      console.log('y/ies detected');
      let rootWord = word.substring(0, word.length - 5);
      pushWord(rootWord + 'y');
      pushWord(rootWord + 'ies');
      continue;
    } else {
      console.log(word + ' does not end with y/ies');
    }

    /**
     * Get root of word before ( / [ symbols
     * The word will be made of only `a-z` `A-Z` `-` `'`
     */
    let rootWord = word.match(/^[a-zA-Z'-]+/)[0];
    pushWord(rootWord);
    /**
     * the stuff after rootWord, with slashes, parentheses, brackets, etc.
     * e.g. `/(r[s])`
     */
    let suffixStructure = word.substring(rootWord.length);
    console.log('suffixStructure: ' + suffixStructure);

    // CASE 3: Additive First-rate(ness[es])/(r[s]) case
    for (let suffix of suffixStructure.split('/')) {
      // Clean parentheses/brackets from start and end of suffix, but not within
      if (suffix[0] === '(' || suffix[0] === '[')
        suffix = suffix.substring(1, suffix.length - 1);

      // In the /(r[s]) structure, the first suffix in split() is blank. Skip.
      if (suffix.length === 0) continue;

      console.log('noOuterParensSuffix: ' + suffix);

      // Extract "root" of suffix from suffixes like "ing(s)"
      let rootSuffix = suffix.match(/^[a-zA-Z'-]+/)[0];

      if (rootWord.endsWith('e') && rootSuffix === 'ing') {
        // CASE 3a: eing words
        pushWord(rootWord.substring(0, rootWord.length - 1) + 'ing');
      } else {
        pushWord(rootWord + rootSuffix);
      }

      // bracket acknowledgement push
      if (suffix.includes('[') || suffix.includes('(')) {
        let afterSuffix = suffix.substring(
          rootSuffix.length + 1,
          suffix.length - 1
        );
        console.log('afterSuffix: ' + afterSuffix);

        if (rootWord.endsWith('e') && rootSuffix === 'ing') {
          // CASE 3b: eings words
          pushWord(rootWord.substring(0, rootWord.length - 1) + 'ings');
        } else {
          pushWord(rootWord + rootSuffix + afterSuffix);
        }
      }
    }
  }

  // print out to one copy-pastable array
  console.log(words.join('\n'));
})();
