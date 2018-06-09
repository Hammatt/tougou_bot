package modules

import "testing"

func TestJishoApi(t *testing.T) {
	//Arrange

	//Act
	jishoResult, err := jishoAPISearch("家")

	//Assert
	if err != nil || jishoResult == "" {
		t.Fatal("Jisho result failure")
	}
}
