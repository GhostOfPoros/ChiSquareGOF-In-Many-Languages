MODULE chimodule
    implicit none
    
    private
    public CRITICAL_VALUES, get_possibility_input, get_float_input, get_chi_sum

    real, dimension(9) :: CRITICAL_VALUES = [3.841, 5.991, 7.815, 9.488, 11.070, 12.592, 14.067, 15.507, 16.919]

    contains

    FUNCTION chi_square_calculation(expected, observed) result(chi_value)
        implicit none
        real, intent(in) :: expected, observed
        
        real :: chi_value

        chi_value = observed - expected
        chi_value = chi_value ** 2
        chi_value = chi_value / expected

    END FUNCTION chi_square_calculation

    FUNCTION get_possibility_input() result(possibilities)
        implicit none
        integer :: possibilities, ierror

        do
            print*, "Please input the number of possibilities (As an integer)"
            read(*,'(i10)',iostat=ierror) possibilities
            
            if ( ierror == 0 ) then
                exit
              endif
              print *, ""
              print *, "Not an integer - please try again"
        end do
    
    END FUNCTION get_possibility_input

    FUNCTION get_float_input(input_text) result(floatnum)
        implicit none
        character*31, intent(in) :: input_text

        integer :: ferror
        real :: floatnum

        do
            print *, ""
            print *, input_text
            read(*,'(G10.0)',iostat=ferror) floatnum

            if ( ferror == 0 ) then
                exit
              endif
              print *, ""
              print *, "Not a float - please try again"
        end do

    END FUNCTION get_float_input

    FUNCTION get_chi_sum(possibilities, expected_value) result(chi_sum)
        implicit none
        
        integer, intent(in) :: possibilities
        real, intent(in) :: expected_value
        real :: chi_sum, observed_value, chi_value
        integer :: i

        chi_sum = 0

        do i = 0, possibilities
            observed_value = get_float_input("Please input the observed value")
            chi_value = chi_square_calculation(expected_value, observed_value)
            chi_sum = chi_sum + chi_value
        end do

    END FUNCTION get_chi_sum

END MODULE chimodule