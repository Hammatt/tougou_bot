package jisho

import "testing"

func TestJishoApi(t *testing.T) {
	//Arrange

	//Act
	jishoResult, err := jishoAPISearch("家")

	//Assert
	if err != nil || jishoResult == "" {
		t.Log("result: " + jishoResult)
		t.Log("error: " + err.Error())
		t.Fatal("Jisho result failure")
	}
}
