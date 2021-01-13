const parseFile = (s) => {
    console.log(`Parsing a file of ${s.length} characters`);
    const re = /(\w+)/g;
    return s
        .match(re)
        .map((w) => {
            return w
                .toLowerCase()
                .replace("_", "") // It's an Alice thing
                .replace("_", "") // Closing too
        });
}

const generateScores = (words) => {
    console.log(`Generating scores for ${words.length} sequences`);
    const predictionMap = {};
    words.forEach((current_word, index) => {
        if (index === words.length) {
            return;
        }
        const key = `${current_word} ${words[index + 1]}`;
        if (!predictionMap[key]) {
            predictionMap[key] = 0;
        }
        predictionMap[key]++;
    });
    return predictionMap;
}

const groupWordPredictions = (predictionsHM) => {
    const hm = Object.create(null);
    for (let key in predictionsHM) {
        const words = key.split(' ');
        const firstWord = words[0];
        const secondWord = words[1];
        if (!hm[firstWord]) {
            hm[firstWord] = [];
        }
        hm[firstWord].push([secondWord, predictionsHM[key]]);
    }

    for (let key in hm) {
        hm[key] = hm[key].sort((a, b) => b[1] - a[1]);
    }
    return hm;
}

export default (s) => groupWordPredictions(generateScores(parseFile));
