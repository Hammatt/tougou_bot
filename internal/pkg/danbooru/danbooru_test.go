package danbooru

import "testing"

func TestDanbooruApi(t *testing.T) {
	//Arrange

	//Act
	danbooruResult, err := danbooruAPISearch("initial_d")

	//Assert
	if err != nil || danbooruResult == "" {
		t.Log("result: " + danbooruResult)
		t.Log("error: " + err.Error())
		t.Fatal("danbooru result failure")
	}
}
