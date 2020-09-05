[*Heroicons*](https://github.com/tailwindlabs/heroicons)をElmから利用できるように変換をするコードをRustの勉強も含めて書いてみた。

すでに同じ目的の[jasonliang512/elm-heroicons](https://package.elm-lang.org/packages/jasonliang512/elm-heroicons/latest/)が存在していたので、公式にはパブリッシュせずにプライベートで利用する。

## 使い方

先のパッケージと同じだが、野良パッケージになるので、[こちらの記事](https://qiita.com/arowM/items/98f47202aa92394d483c)にならって 使いたいElmのプロジェクト内で、`elm.json`の`source-directories`にこのディレクトリを追加します。

公開されているモジュールは`Heroicons.Outline`と`Heroicons.Solid`になります。各アイコンに相当する関数は引数として`List (Svg.Attribute msg)`をとります。ここに`Svg.Attributes.class`や`Svg.Attributes.style`を記載してアイコンの色やサイズを変更します。

**注意** `Html.Attributes.class`などを渡すとパニックになリます。

## Elmコードの生成方法

genディレクトリで

```sh
% ./from_repo.sh
```

とするとElmのコードを生成してelm-formatで整形してくれます。

`elm-format`とRustコンパイラ一式がインストール済みであることが前提です。

