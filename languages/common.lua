-- Common utilities for Lua language profiles
-- This module provides shared functions and utilities for language profile creation

common = {}

-- Generate syllable patterns from components and lengths
function common.generate_patterns(components, lengths)
    local patterns = {}
    
    for _, length in ipairs(lengths) do
        -- Generate all combinations of components with given length
        local function generate_combinations(components, length, current, all_combinations)
            if length == 0 then
                table.insert(all_combinations, current)
                return
            end
            
            for _, component in ipairs(components) do
                generate_combinations(components, length - 1, current .. component, all_combinations)
            end
        end
        
        local combinations = {}
        generate_combinations(components, length, "", combinations)
        
        for _, pattern in ipairs(combinations) do
            table.insert(patterns, {
                pattern = pattern,
                frequency = common.calculate_pattern_frequency(pattern)
            })
        end
    end
    
    return patterns
end

-- Calculate pattern frequency based on pattern complexity
function common.calculate_pattern_frequency(pattern)
    local base_frequency = 1.0
    local length = string.len(pattern)
    
    -- Simple patterns are more common
    if length <= 2 then
        return base_frequency * 0.8
    elseif length == 3 then
        return base_frequency * 0.6
    elseif length == 4 then
        return base_frequency * 0.4
    else
        return base_frequency * 0.2
    end
end

-- Create a phoneme with standard structure
function common.create_phoneme(ipa, phoneme_type, grapheme, frequency)
    return {
        ipa = ipa,
        phoneme_type = phoneme_type,
        grapheme = grapheme,
        frequency = frequency or 0.5
    }
end

-- Create a phoneme cluster
function common.create_cluster(phonemes, frequency)
    return {
        phonemes = phonemes,
        frequency = frequency or 0.5
    }
end

-- Apply simple vowel harmony based on first vowel
function common.apply_vowel_harmony(syllables, front_vowels, back_vowels)
    if #syllables == 0 then
        return
    end
    
    local first_syllable = syllables[1]
    if #first_syllable.nucleus == 0 then
        return
    end
    
    local first_vowel = first_syllable.nucleus[1]
    local is_front = false
    
    -- Check if first vowel is front
    for _, vowel in ipairs(front_vowels) do
        if first_vowel == vowel then
            is_front = true
            break
        end
    end
    
    -- Apply harmony to subsequent syllables
    for i = 2, #syllables do
        if #syllables[i].nucleus > 0 then
            local current_vowel = syllables[i].nucleus[1]
            
            if is_front then
                -- Convert back vowels to front
                for j, back_vowel in ipairs(back_vowels) do
                    if current_vowel == back_vowel then
                        if front_vowels[j] then
                            syllables[i].nucleus[1] = front_vowels[j]
                        end
                        break
                    end
                end
            else
                -- Convert front vowels to back
                for j, front_vowel in ipairs(front_vowels) do
                    if current_vowel == front_vowel then
                        if back_vowels[j] then
                            syllables[i].nucleus[1] = back_vowels[j]
                        end
                        break
                    end
                end
            end
        end
    end
end

-- Apply vowel reduction in unstressed syllables
function common.apply_vowel_reduction(syllables, schwa, reduction_strength)
    reduction_strength = reduction_strength or 0.8
    
    for i, syllable in ipairs(syllables) do
        if not syllable.stressed and #syllable.nucleus > 0 then
            -- Reduce vowel to schwa with given probability
            if math.random() < reduction_strength then
                syllable.nucleus[1] = schwa
            end
        end
    end
end

-- Apply consonant cluster simplification
function common.simplify_consonant_clusters(syllables, max_cluster_size)
    max_cluster_size = max_cluster_size or 2
    
    for i, syllable in ipairs(syllables) do
        -- Simplify onset clusters
        if #syllable.onset > max_cluster_size then
            local new_onset = {}
            for j = 1, max_cluster_size do
                table.insert(new_onset, syllable.onset[j])
            end
            syllable.onset = new_onset
        end
        
        -- Simplify coda clusters
        if #syllable.coda > max_cluster_size then
            local new_coda = {}
            for j = 1, max_cluster_size do
                table.insert(new_coda, syllable.coda[j])
            end
            syllable.coda = new_coda
        end
    end
end

-- Copy a table (deep copy)
function common.table_copy(original)
    local copy = {}
    for key, value in pairs(original) do
        if type(value) == "table" then
            copy[key] = common.table_copy(value)
        else
            copy[key] = value
        end
    end
    return copy
end

-- Helper function to check if a table contains a value
function common.table_contains(table, value)
    for _, v in ipairs(table) do
        if v == value then
            return true
        end
    end
    return false
end

-- Debug function to print table contents
function common.print_table(t, indent)
    indent = indent or 0
    local prefix = string.rep("  ", indent)
    
    for key, value in pairs(t) do
        if type(value) == "table" then
            print(prefix .. key .. ":")
            common.print_table(value, indent + 1)
        else
            print(prefix .. key .. ": " .. tostring(value))
        end
    end
end

return common