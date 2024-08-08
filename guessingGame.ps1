$random = New-Object System.Random
$lower = 0
$higher = 0
$guesses = 5
$userGuess = 0
$randNum = 0

while($true) {
	$lower = Read-Host "What is the lower number?"
	if($lower -lt 0) {
		Write-Host "Your number is too low"
	}
	else {
		break
	}
}

while($true) {
	$higher = Read-Host "What is the higher number?"
	if($higher -lt $lower) {
		Write-Host "Your number is too low"
	}
	else {
		break
	}
}

$randNum = $random.Next($lower, $higher)

Write-Host "The range is between $lower and $higher"
while($guesses -gt 0) {
	Write-Host "You have $guesses guesses left"
	$userGuess = Read-Host "What is your guess? "
	$guesses -= 1
	if($userGuess -lt $randNum) {
		Write-Host "Your number is too low"
	}
	elseif($userGuess -gt $randNum) {
		Write-Host "Your number is too high"
	}
	else{
		Write-Host "You guessed the number"
		break
	}
	if($guesses -eq 0) {
		Write-Host "You did not guess the number"
		Write-Host "The number was $randNum"
	}
}