# Bakery Text

## EN
![image](https://github.com/user-attachments/assets/dfffdeb3-b7d2-4651-8379-42ec01cbe8f6)

A command-line bakery management simulation focused on inventory, maintenance, security, hygiene, and cost efficiency.

### Basic Commands

- `help`: Display available commands in the current section.
- `rest`: Halt equipment temporarily to recover durability.
- `work`: Resume operations after rest.
- `dispose`: Transfer waste to the Waste Station.

### NIGIWAI

- A 5-star metric affecting demand and pricing:
  - ⭐️ 5.0: High demand and prices; high ingredient cost.
  - ⭐️ 0.0: Low demand and prices; low ingredient cost.
- Influenced by:
  - ✅ Proper stock and waste management
  - ✅ Cleanliness and intruder control
  - ✅ Stable output and high sales
  - ❌ Overcrowding and excessive waste
  - ❌ Unaddressed intruders (rats, thieves)
  - ❌ Complaints or refunds

### Departments & Commands

| ID  | Mode          | Abbr | Function                            |
|-----|---------------|------|-------------------------------------|
| 00  | Purchasing    | PS   | Procure raw materials               |
| 01  | Pantry        | PN   | Monitor ingredient and product status |
| 02  | Mixing        | MX   | Auto: Convert ingredients to dough.<br>Use `plan` to manage production plan. |
| 03  | Cooling       | CL   | Auto: Primary/secondary fermentation |
| 04  | Shaping       | SH   | Auto: Shape fermented dough         |
| 05  | Baking        | BK   | Auto: Bake shaped dough             |
| 06  | Packaging     | PK   | Use `seal` to finalize baked items  |
| 07  | Sales Front   | SF   | Auto: Sales occur probabilistically |
| 08  | Waste Station | WS   | Waste destination                   |
| 09  | Utilities     | UT   | Manage water, electricity, gas      |

#### Production Plan (MX)

- Use `plan` to review or modify the bread types and quantities per batch:
  - `plan`: Display current plan (types and quantities per batch)
  - `plan anpan 1`: Adjust plan to produce 1 Anpan per batch

### Department Issues

- **PS**: Price spikes, fraud, delays (weather, logistics)
- **PN**: Spoilage, pest invasion
- **MX/CL/SH/BK**: Equipment failure, work accidents
- **BK**: Temperature anomalies, gas leaks
- **PK**: Foreign object contamination
- **SF**: Complaints, robbery
- **WS**: Excessive waste buildup
- **UT**: Water or electrical leakage

### Hygiene & Intruders

- **Nezumi (Rats) 🐭**
  - **Effect**: Consume inventory; risk of inspection
  - **Actions**:
    - `trap`: Set traps to catch rats

### Game Over Conditions

Operation becomes unsustainable when any of the following occur → **Game Over**:

- Room temperature drops below 10°C or exceeds 38°C
- Equipment wear reaches 100%
- Waste capacity reaches 100% (excluding Waste Station)
- More than 5 rats detected during a health inspection event
- 5 or more departments (out of 9) report active issues (excluding those on rest)

### Bread Recipes & Prices

| Ingredient      | Unit Price (¥/g) | A (Anpan) | B (Baguette) | C (Croissant) | N (Naan) |
|----------------|------------------|-----------|--------------|---------------|----------|
| Flour          | 0.2              | 40        | 133          | 35.7          | 50       |
| Yeast          | 15.0             | 0.6       | 0.2          | 1.4           | 1        |
| Salt           | 0.1              | 0.6       | 2.7          | 0.7           | 1        |
| Butter         | 2.4              | 4         | —            | 18.6          | —        |
| Sugar          | 0.3              | 5         | —            | 2.5           | 2        |
| Milk           | 0.2              | 2         | —            | 24.3          | —        |
| Red Bean       | 1.1              | 30        | —            | —             | —        |
| Malt           | 2.3              | —         | 1            | —             | —        |
| **Selling Price** | —             | 104       | 53           | 135           | 42       |
| **Gross Margin** | —              | **68.9%** | **64.7%**    | **72.1%**     | **63.4%** |

## JP
![bt-ja](https://github.com/user-attachments/assets/66f82496-3fac-42ca-ba0a-128e9ad98e78)

在庫・設備・防犯・衛生・コストに重点を置いた、CUIベースのベーカリー経営シミュレーション。

### 基本コマンド

- `help`：現在のセクションで使用可能なコマンドを表示
- `rest`：機器の稼働を一時停止し、摩耗を回復
- `work`：休止中の機器を再稼働
- `dispose`：廃棄物を廃棄ステーションに転送

### NIGIWAI（にぎわい）

需要・価格に影響する5段階評価：

- ⭐️ 5.0：需要・価格ともに高水準。ただし原料価格も高騰
- ⭐️ 0.0：需要・価格は低水準。原料価格は安価

影響要素：

- ✅ 適切な在庫／廃棄管理
- ✅ 衛生状態・侵入者対策
- ✅ 安定供給と高売上
- ❌ 過密／廃棄過多
- ❌ 侵入者（ネズミ・泥棒）の放置
- ❌ クレーム・返金対応

### 部門一覧と機能

| ID  | モード     | 略称 | 機能                          |
|-----|------------|------|-------------------------------|
| 00  | 購入       | PS   | 原材料の調達                 |
| 01  | 保管庫     | PN   | 原料・製品の状態確認         |
| 02  | 混合       | MX   | 自動：原料を生地に変換。<br>`plan` で製造計画を確認・調整可能 |
| 03  | 冷却       | CL   | 自動：発酵処理               |
| 04  | 成形       | SH   | 自動：生地を成形             |
| 05  | 焼成       | BK   | 自動：生地を焼成             |
| 06  | 包装       | PK   | `seal`：製品の包装           |
| 07  | 販売       | SF   | 自動：確率的に販売（NIGIWAI依存）|
| 08  | 廃棄       | WS   | 廃棄物の転送先               |
| 09  | 設備       | UT   | 水道・電気・ガスの管理       |

#### 製造計画(MX)

- `plan`：1回の混錬あたりのパンの種類と数を確認・調整可能
  - `plan`：現在の製造計画を表示
  - `plan anpan 1`：アンパンの製造数を1個に変更


### 部門別トラブル

- **PS（購入）**：価格変動、詐欺、配送遅延（天候・交通）
- **PN（保管庫）**：腐敗、害虫
- **MX/CL/SH/BK**：設備故障、作業事故
- **BK（焼成）**：温度異常、ガス漏れ
- **PK（包装）**：異物混入
- **SF（販売）**：クレーム、強盗
- **WS（廃棄）**：廃棄物過多
- **UT（設備）**：水漏れ、漏電

### 衛生・侵入者

- **ネズミ（Nezumi）🐭**
  - **影響**：在庫破損・衛生悪化・摘発リスク
  - **対策**：
    - `trap`：罠を設置し捕獲

### ゲームオーバー条件

以下のいずれかを満たすと営業継続不可 → **ゲームオーバー**：

- 室温が10°C未満または38°C超過
- 設備摩耗率が100%に到達
- 廃棄容量が100%に到達（WS除く）
- ネズミが5匹以上の状態で保健所イベント発生
- 9部門中5部門以上でトラブル（休止中は除く）

### パンのレシピと価格

| 材料         | 単価（¥/g） | A（あんぱん） | B（バゲット） | C（クロワッサン） | N（ナン） |
|--------------|--------------|----------------|----------------|---------------------|------------|
| 小麦粉        | 0.2          | 40             | 133            | 35.7                | 50         |
| イースト      | 15.0         | 0.6            | 0.2            | 1.4                 | 1          |
| 塩            | 0.1          | 0.6            | 2.7            | 0.7                 | 1          |
| バター        | 2.4          | 4              | —              | 18.6                | —          |
| 砂糖          | 0.3          | 5              | —              | 2.5                 | 2          |
| 牛乳          | 0.2          | 2              | —              | 24.3                | —          |
| あんこ        | 1.1          | 30             | —              | —                   | —          |
| モルト        | 2.3          | —              | 1              | —                   | —          |
| **販売価格**  | —            | 104            | 53             | 135                 | 42         |
| **粗利率**    | —            | **68.9%**      | **64.7%**      | **72.1%**           | **63.4%**  |
