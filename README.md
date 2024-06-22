# 循環動態シミュレーション数理モデル

## 概要

このプロジェクトは、人体の循環動態をシミュレーションする高度な数理モデルを実装したものです。心臓の収縮と弛緩、血液の流れ、圧力の変化などを数学的に表現し、様々な生理的状態や病態における循環動態を解析することができます。

## 特徴

- 心臓（右房、右室、左房、左室）、肺循環、体循環を含む包括的なモデル
- 電気回路アナロジーを用いた直感的な表現
- 時変エラスタンス関数による心臓の収縮表現
- 弁機能のモデル化による一方向血流の再現
- 遺伝的アルゴリズムを用いたパラメータ最適化機能

## モデル構造

このモデルは以下の主要コンポーネントで構成されています：

1. 4つの心腔（RA, RV, LA, LV）
2. 4つの心臓弁（三尖弁, 肺動脈弁, 僧帽弁, 大動脈弁）
3. 体循環系（動脈、静脈、毛細血管床）
4. 肺循環系（動脈、静脈、毛細血管床）
5. 近位および遠位血管セグメント

各コンポーネントは抵抗（R）とコンプライアンス（C）の組み合わせでモデル化されています。

## 主要な数式

時変エラスタンス関数:
\[
e(t) = \begin{cases} 
\frac{1}{2}\left(\sin\left(\frac{\pi t}{T_{max}} - \frac{\pi}{2}\right) + 1\right)(1-b) + b & \text{if } 0 \leq t < T_{max} \\
e^{-(t-T_{max})/\tau}(1-b) + b & \text{if } T_{max} \leq t < \frac{3T_{max}}{2} \\
b & \text{if } \frac{3T_{max}}{2} \leq t < \frac{60000}{HR}
\end{cases}
\]

心腔内圧力:
\[
P(V,t) = P_{ed}(V) + e(t-AV_{delay})(P_{es}(V) - P_{ed}(V))
\]

## API使用方法

このモデルはRESTful APIを通じて利用可能です。以下は基本的な使用例です：

```bash
curl -X POST https://api-endpoint/optimize \
-H "Content-Type: application/json" \
-d '{
    "target_metrics": [
        [52.5, "stroke_volume", 1.0],
        [10.3, "central_venous_pressure", 20.0],
        [20.7, "pulmonary_capillary_wedge_pressure", 4.0],
        [126.1, "systolic_arterial_pressure", 0.8],
        [69.5, "diastolic_arterial_pressure", 1.0],
        [46.6, "systolic_pulmonary_arterial_pressure", 2.0],
        [21.1, "diastolic_pulmonary_arterial_pressure", 2.0],
        [55.0, "left_ventricular_ejection_fraction", 1.0]
    ],
    "param_updates": {
        "HR": [80.0, null, false]
    },
    "num_repeats": 1
}'
```

## 応用例

* 心不全患者の病態理解と治療戦略の検討
* 循環器系薬剤の効果予測
* 手術前後の循環動態変化の予測
* 生理学的研究における仮説検証

## 注意事項

このモデルは簡略化された表現であり、実際の生体システムのすべての複雑さを捉えているわけではありません。
パラメータの最適化結果は、与えられた目標値と初期条件に大きく依存します。
モデルの妥当性は、常に実験データや臨床データとの比較によって検証する必要があります。