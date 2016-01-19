//! Corrections for atmospheric refraction

use angle;

/**
Returns the **refraction term** for true altitude

This method is valid only when the altitude of the body
is more than 15 degrees.

# Returns

* ```refrac```: The refraction term *| in radians*, that needs to be
                subtracted from the apparent altitude to get the
                true altitude

# Arguments

* ```apprnt_alt```: Apparent altitude *| in radians*
**/
pub fn RefracFrmApprntAlt(apprnt_alt: f64) -> f64 {
      angle::DegFrmDMS(0, 0, 58.294).to_radians()*(90_f64.to_radians() - apprnt_alt).tan()
    - angle::DegFrmDMS(0, 0, 0.0668).to_radians()*(90_f64.to_radians() - apprnt_alt).tan().powi(3)
}

/**
Returns the **refraction term** for apparent altitude

This method is valid only when the altitude of the body
is more than 15 degrees.

# Returns

* ```refrac```: The refraction term *| in radians*, that needs to be
                added to the true altitude to get the apparent altitude

# Arguments

* ```true_alt```: True altitude *| in radians*
**/
pub fn RefracFrmTrueAlt(true_alt: f64) -> f64 {
      angle::DegFrmDMS(0, 0, 58.276).to_radians()*(90_f64.to_radians() - true_alt).tan()
    - angle::DegFrmDMS(0, 0, 0.0824).to_radians()*(90_f64.to_radians() - true_alt).tan().powi(3)
}

/**
Returns the **approximate refraction term** for true altitude

This method is valid for all values of altitude from 0 to 90 degrees;
the accuracy is upto 0.07 arcminute for all values of apparent
altitude.

# Returns

* ```refrac```: The refraction term *| in radians*, that needs to be
                subtracted from the apparent altitude to get the
                true altitude

# Arguments

* ```apprnt_alt```: Apparent altitude *| in radians*
**/
pub fn ApproxRefracFrmApprntAlt(apprnt_alt: f64) -> f64 {
    if apprnt_alt.to_degrees() == 90.0 { 0.0 }
    else {
        let a =   apprnt_alt.to_degrees()
                + 7.31 / (apprnt_alt.to_degrees() + 4.4);
        let R = 1.0 / a.to_radians().tan();

        (R / 60.0).to_radians()
    }
}

/**
Returns the **approximate refraction term** for apparent altitude

This method is valid for all values of altitude from 0 to 90 degrees;
it is consistent with ```ApproxRefracFrmApprntAlt()``` to within
4 arcseconds.

# Returns

* ```refrac```: The refraction term *| in radians*, that needs to be
                added to the true altitude to get the apparent altitude

# Arguments

* ```true_alt```: True altitude *| in radians*
**/
pub fn ApproxRefracFrmTrueAlt(true_alt: f64) -> f64 {
    if true_alt.to_degrees() == 90.0 { 0.0 }
    else {
        let a = (   true_alt.to_degrees()
                  + 10.3 / (true_alt.to_degrees() + 5.11)
                );
        let R =   1.02 / a.to_radians().tan();

        (R / 60.0).to_radians()
    }
}

/**
Returns the **refraction term modifier** for local pressure

# Returns

* ```refrac_term_modifier```: The value that needs to be multiplied by the
                              refraction term to account for local pressure

# Arguments

* ```pressure```: Local pressure (*millibars*)
**/
pub fn RefracDueToPressure(pressure: f64) -> f64 {
    pressure / 1010.0
}

/**
Returns the **refraction term modifier** for local temperature

# Returns

* ```refrac_term_modifier```: The value that needs to be multiplied by the
                              refraction term to account for local temperature

# Arguments

* ```temp```: Local temperature (*kelvins*)
**/
pub fn RefracDueToTemp(temp: f64) -> f64 {
    283.0 / temp
}
