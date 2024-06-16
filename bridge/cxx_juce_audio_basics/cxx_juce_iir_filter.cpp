#include "cxx_juce_iir_filter.h"

namespace cxx_juce::iir_filter
{
std::unique_ptr<juce::SingleThreadedIIRFilter> createIIRFilter (std::array<rust::f32, 5> coefficients)
{
    auto filter = std::make_unique<juce::SingleThreadedIIRFilter>();

    juce::IIRCoefficients coeffs;
    coeffs.coefficients[0] = coefficients[0];
    coeffs.coefficients[1] = coefficients[1];
    coeffs.coefficients[2] = coefficients[2];
    coeffs.coefficients[3] = coefficients[3];
    coeffs.coefficients[4] = coefficients[4];

    filter->setCoefficients (coeffs);

    return filter;
}

std::array<rust::f32, 5> makeLowPass (double sampleRate,
                                      double cutoffFrequency,
                                      double q)
{
    const auto coefficients = juce::IIRCoefficients::makeLowPass (
        sampleRate,
        cutoffFrequency,
        q);

    return { coefficients.coefficients[0],
             coefficients.coefficients[1],
             coefficients.coefficients[2],
             coefficients.coefficients[3],
             coefficients.coefficients[4] };
}

std::array<rust::f32, 5> makeHighPass (double sampleRate,
                                       double cutoffFrequency,
                                       double q)
{
    const auto coefficients = juce::IIRCoefficients::makeHighPass (
        sampleRate,
        cutoffFrequency,
        q);

    return { coefficients.coefficients[0],
             coefficients.coefficients[1],
             coefficients.coefficients[2],
             coefficients.coefficients[3],
             coefficients.coefficients[4] };
}

std::array<rust::f32, 5> makeNotchFilter (double sampleRate,
                                          double cutoffFrequency,
                                          double q)
{
    const auto coefficients = juce::IIRCoefficients::makeNotchFilter (
        sampleRate,
        cutoffFrequency,
        q);

    return { coefficients.coefficients[0],
             coefficients.coefficients[1],
             coefficients.coefficients[2],
             coefficients.coefficients[3],
             coefficients.coefficients[4] };
}
} // namespace cxx_juce::iir_filter
