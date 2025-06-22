# AI

<!-- toc -->

## **Regression evaluation metrics**

## 1. Mean Absolute Error (MAE)

- **What it measures**:  
The average *absolute* difference between predicted and actual values.

  - Ignores direction (over- or under-prediction).
  - Treats all errors equally.
- **Calculation**:
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block"><semantics><mrow><mtext>MAE</mtext><mo>=</mo><mfrac><mrow><mi mathvariant="normal">∣</mi><mn>2</mn><mi mathvariant="normal">∣</mi><mo>+</mo><mi mathvariant="normal">∣</mi><mn>3</mn><mi mathvariant="normal">∣</mi><mo>+</mo><mi mathvariant="normal">∣</mi><mn>3</mn><mi mathvariant="normal">∣</mi><mo>+</mo><mi mathvariant="normal">∣</mi><mn>1</mn><mi mathvariant="normal">∣</mi><mo>+</mo><mi mathvariant="normal">∣</mi><mn>2</mn><mi mathvariant="normal">∣</mi><mo>+</mo><mi mathvariant="normal">∣</mi><mn>3</mn><mi mathvariant="normal">∣</mi></mrow><mn>6</mn></mfrac><mo>=</mo><mfrac><mn>14</mn><mn>6</mn></mfrac><mo>≈</mo><mn>2.33</mn></mrow></semantics></math>MAE=6∣2∣+∣3∣+∣3∣+∣1∣+∣2∣+∣3∣​=614​≈2.33
  - **Interpretation**: On average, predictions are off by **2.33 ice creams**.
- **When to use**:

  - When all errors (small or large) should be weighted equally.
  - Preferred when outliers are *not* a critical concern.

* * *

## 2. Mean Squared Error (MSE)

- **What it measures**:  
The average of the *squared* differences between predicted and actual values.

  - Emphasizes larger errors (since squaring amplifies them).
- **Calculation**:
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block"><semantics><mrow><mtext>MSE</mtext><mo>=</mo><mfrac><mrow><msup><mn>2</mn><mn>2</mn></msup><mo>+</mo><msup><mn>3</mn><mn>2</mn></msup><mo>+</mo><msup><mn>3</mn><mn>2</mn></msup><mo>+</mo><msup><mn>1</mn><mn>2</mn></msup><mo>+</mo><msup><mn>2</mn><mn>2</mn></msup><mo>+</mo><msup><mn>3</mn><mn>2</mn></msup></mrow><mn>6</mn></mfrac><mo>=</mo><mfrac><mrow><mn>4</mn><mo>+</mo><mn>9</mn><mo>+</mo><mn>9</mn><mo>+</mo><mn>1</mn><mo>+</mo><mn>4</mn><mo>+</mo><mn>9</mn></mrow><mn>6</mn></mfrac><mo>=</mo><mfrac><mn>36</mn><mn>6</mn></mfrac><mo>=</mo><mn>6</mn></mrow></semantics></math>MSE=622+32+32+12+22+32​=64+9+9+1+4+9​=636​=6
  - **Interpretation**: The "score" is **6**, but this isn’t in ice cream units (due to squaring).
- **When to use**:

  - When large errors are particularly undesirable (e.g., in financial models).
  - Used as a loss function in machine learning (optimization).

* * *

## 3. Root Mean Squared Error (RMSE)

- **What it measures**:  
The square root of MSE, converting the metric back to the original units (ice creams).
- **Calculation**:
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block"><semantics><mrow><mtext>RMSE</mtext><mo>=</mo><msqrt><mtext>MSE</mtext></msqrt><mo>=</mo><msqrt><mn>6</mn></msqrt><mo>≈</mo><mn>2.45</mn></mrow></semantics></math><svg xmlns="http://www.w3.org/2000/svg" width="400em" height="1.08em" viewbox="0 0 400000 1080" preserveaspectratio="xMinYMin slice"></svg>​

- **Interpretation**: Predictions are off by **~2.45 ice creams** on average, with larger errors weighted more heavily.
- **When to use**:

  - When you want MSE’s sensitivity to large errors *but* need interpretability in original units.

* * *

### **Key Differences Summary**

| Metric | Units          | Outlier Sensitivity | Interpretation                     |
|--------|----------------|---------------------|------------------------------------|
| MAE    | Original (e.g., ice creams) | Low          | "Average error is X units"         |
| MSE    | Squared (e.g., ice creams²) | High         | "Squared error average is X"       |
| RMSE   | Original (e.g., ice creams) | High         | "Error magnitude is ~X units"      |

### **Ice Cream Example Recap**

- **MAE (2.33)**: "On average, predictions are off by ~2.3 ice creams."
- **MSE (6)**: "Large errors are penalized; score is 6 (no unit meaning)."
- **RMSE (2.45)**: "After squaring and square-rooting, errors average ~2.5 ice creams, with larger mistakes weighted more."

**Which to choose?**

- Use **MAE** for straightforward, outlier-resistant interpretation.
- Use **RMSE** if large errors are critical (e.g., safety-critical systems).

### When to Use

- **MAE**: When all errors should be treated equally  
- **MSE/RMSE**: When large errors are particularly undesirable  
- **RMSE**: Preferred when you need interpretability in original units

## Coefficient of Determination (R²)

### Definition

R² (R-Squared) measures the **proportion of variance** in the dependent variable (e.g., ice cream sales) that is predictable from the independent variable(s) in a regression model. It quantifies how well the model explains the observed data.

* * *

### Key Formula

$$
R^2 = 1 - \frac{\sum (y - \hat{y})^2}{\sum (y - \bar{y})^2}
$$

- \( y \): Actual values  
- \( \hat{y} \): Predicted values  
- \( \bar{y} \): Mean of actual values actual values  

* * *

### Interpretation

- **R² = 1**: Perfect fit (100% variance explained).  
- **R² = 0**: Model explains none of the variance (no better than predicting the mean).  
- **R² < 0**: Model performs worse than a horizontal line (rare, indicates severe issues).  

#### Ice Cream Example

- **R² = 0.95**:  
  - The model explains **95%** of the variance in ice cream sales.  
  - Only **5%** of the variance is due to random/unexplained factors (e.g., festivals, weather anomalies).  

* * *

### Comparison with Other Metrics

| Metric       | Purpose                          | Scale      | Focus                     |
|--------------|----------------------------------|------------|---------------------------|
| **MAE**      | Average absolute error           | Original   | Magnitude of errors       |
| **MSE/RMSE** | Squared/root-squared error       | Squared    | Penalizes large errors    |
| **R²**       | Proportion of variance explained | 0 to 1     | Model explanatory power   |

* * *

### Why It Matters

1. **Contextualizes Error**:  
   - Unlike MAE/MSE, R² compares errors to natural variance in the data.  
2. **Model Selection**:  
   - Higher R² indicates a better-fit model (but beware of overfitting!).  
3. **Intuitive Scale**:  
   - 0–1 range simplifies communication of model performance.  

* * *

### Limitations

- **Not for All Models**: Misleading for non-linear relationships.  
- **Overfitting Risk**: Adding variables can artificially inflate R².  
- **No Direction**: Doesn’t indicate if predictions are biased.  

> **Note**: Use R² alongside other metrics (e.g., RMSE) for a complete evaluation.

