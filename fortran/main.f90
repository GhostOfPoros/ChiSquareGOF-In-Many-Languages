! DATA DICTIOARY:
! CRITICAL_VALUES: A constant array that contains the critical valuess
!   corresponding to the number of possibilities
! possibilities: number of different categories. Ex: six sided die has possibility of 2
! Expected value: Number of successes for a category in a perfect world. Ex: six sided 
!   die rolled 36 times has an expected value of 6 for each side of the die.

PROGRAM chi_square
    use chimodule
    implicit none

    integer :: possibilities
    real :: expected_value, chi_sum
!   real, external :: get_chi_sum

    print *, "This program determines if the null hypothesis is accepted or rejected"
    print *, "when the p-value is 0.05"
    print *, ""

    possibilities = get_possibility_input()

    expected_value = get_float_input("Please input the expected value")

    chi_sum = get_chi_sum(possibilities, expected_value)

    print *, ""
    print *, "The expected value is ", expected_value
    print *, "The number of possibilities is ", possibilities
    print *, "Chi Sum: ", chi_sum

    if (chi_sum > CRITICAL_VALUES(possibilities)) then
        print *, "Null hypothesis (H0) is rejected"
    else
        print *, "Null hypothesis (H0) is accepted"
    endif

STOP
END PROGRAM chi_square