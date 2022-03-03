안녕하세요, Roy의 지인 Danuel입니다.

OpenSurvey에는 내부 세미나 끝에 다음 발표자를 추첨하는 전통이 생기고 있다고 하면서 이 저장소를 소개해주더라구요.
그렇게 살펴 보던 중 몇몇 궁금한 부분이 있어 이슈를 만들어 문의해 봅니다.
(Roy에게 확인해보니 이후에는 새로 만들 가능성이 크다는 것을 알았지만요...)

1. [@emotion/react](https://www.npmjs.com/package/@emotion/react)를 사용하지 않고 [@emotion/css](https://emotion.sh/docs/@emotion/css)만으로 작성한 이유를 알고 싶습니다.

1. (아래와 링크한 부분처럼)해당 컴포넌트 바깥에서는 쓰이지 않는 CSS를 export 하는 것은 코딩 컨벤션일까요?
   https://github.com/chinsun9/draw-lots/blob/a38523b0b42034c86062424d63fbe351c0139be3/src/NextButton.tsx#L58-L63

1. components/ 디렉토리가 있지만 `<NextButton>`, `<Result>` 컴포넌트는 components/ 디렉토리 밖에 위치하더라구요.
   다른 의미가 있어서 그런 것일까요?

1. shuffle 함수는 [라이브러리](https://github.com/sindresorhus/array-shuffle)로 불러다 쓸 수 있는데 직접 내부 코드로 가져온 것은 [fisher-yates 알고리즘](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle)에 추가적인 개선을 해보려는 것이었을까요?
   https://github.com/chinsun9/draw-lots/blob/a38523b0b42034c86062424d63fbe351c0139be3/src/utils/index.ts#L1-L11

1. getRandomInt 보다는 [randRange](https://grep.app/search?q=randRange%28)(혹은 [randomRange](https://grep.app/search?q=randomRange%28))를 제안해 봅니다.
   getRandomInt라고 하면 get으로 시작해서 어떤 자료구조/객체의 메서드인 것 같은 느낌이더라구요. 표현하자면... Random.getRandomInt 처럼 써야 할 것 같은 느낌?
   https://github.com/chinsun9/draw-lots/blob/a38523b0b42034c86062424d63fbe351c0139be3/src/utils/index.ts#L13-L14

감사합니당!
