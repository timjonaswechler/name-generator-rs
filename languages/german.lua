-- German Language Profile
-- Lua-based German language profile with dynamic phonetic rules

local profile = {}

profile.name = "German"

-- Define phonemes programmatically
profile.phonemes = {
    -- Consonants
    { ipa = "[p]", phoneme_type = "Consonant", grapheme = "p", frequency = 0.3 },
    { ipa = "[b]", phoneme_type = "Consonant", grapheme = "b", frequency = 0.3 },
    { ipa = "[t]", phoneme_type = "Consonant", grapheme = "t", frequency = 0.3 },
    { ipa = "[d]", phoneme_type = "Consonant", grapheme = "d", frequency = 0.3 },
    { ipa = "[k]", phoneme_type = "Consonant", grapheme = "k", frequency = 0.3 },
    { ipa = "[g]", phoneme_type = "Consonant", grapheme = "g", frequency = 0.3 },
    { ipa = "[f]", phoneme_type = "Consonant", grapheme = "f", frequency = 0.3 },
    { ipa = "[v]", phoneme_type = "Consonant", grapheme = "w", frequency = 0.3 },
    { ipa = "[s]", phoneme_type = "Consonant", grapheme = "s", frequency = 0.3 },
    { ipa = "[z]", phoneme_type = "Consonant", grapheme = "s", frequency = 0.4 },
    { ipa = "[ʃ]", phoneme_type = "Consonant", grapheme = "sch", frequency = 0.4 },
    { ipa = "[ç]", phoneme_type = "Consonant", grapheme = "ch", frequency = 0.3 },
    { ipa = "[h]", phoneme_type = "Consonant", grapheme = "h", frequency = 0.4 },
    { ipa = "[m]", phoneme_type = "Consonant", grapheme = "m", frequency = 0.2 },
    { ipa = "[n]", phoneme_type = "Consonant", grapheme = "n", frequency = 0.4 },
    { ipa = "[l]", phoneme_type = "Consonant", grapheme = "l", frequency = 0.3 },
    { ipa = "[ʁ]", phoneme_type = "Consonant", grapheme = "r", frequency = 0.2 },
    { ipa = "[j]", phoneme_type = "Consonant", grapheme = "j", frequency = 0.3 },

    -- Vowels
    { ipa = "[a]", phoneme_type = "Vowel", grapheme = "a", frequency = 0.9 },
    { ipa = "[ɛ]", phoneme_type = "Vowel", grapheme = "e", frequency = 0.8 },
    { ipa = "[e]", phoneme_type = "Vowel", grapheme = "e", frequency = 0.7 },
    { ipa = "[ɪ]", phoneme_type = "Vowel", grapheme = "i", frequency = 0.8 },
    { ipa = "[i]", phoneme_type = "Vowel", grapheme = "i", frequency = 0.7 },
    { ipa = "[ɔ]", phoneme_type = "Vowel", grapheme = "o", frequency = 0.6 },
    { ipa = "[o]", phoneme_type = "Vowel", grapheme = "o", frequency = 0.5 },
    { ipa = "[ʊ]", phoneme_type = "Vowel", grapheme = "u", frequency = 0.6 },
    { ipa = "[u]", phoneme_type = "Vowel", grapheme = "u", frequency = 0.5 },
    { ipa = "[ə]", phoneme_type = "Vowel", grapheme = "e", frequency = 0.6 },

    -- Diphthongs
    { ipa = "[aɪ]", phoneme_type = "Vowel", grapheme = "ei", frequency = 0.5 },
    { ipa = "[aʊ]", phoneme_type = "Vowel", grapheme = "au", frequency = 0.4 },
}

-- Dynamic syllable patterns
profile.syllable_patterns = {
    { pattern = "CV",   frequency = 0.3 },
    { pattern = "CVC",  frequency = 0.4 },
    { pattern = "VC",   frequency = 0.1 },
    { pattern = "V",    frequency = 0.05 },
    { pattern = "CCVC", frequency = 0.1 },
    { pattern = "CVCC", frequency = 0.05 },
}

-- Define phoneme groups
profile.phoneme_groups = {
    front_vowels = { "[i]", "[ɪ]", "[e]", "[ɛ]" },
    back_vowels = { "[u]", "[ʊ]", "[o]", "[ɔ]", "[a]" },
    central_vowels = { "[ə]" },
    diphthongs = { "[aɪ]", "[aʊ]" },
    plosives = { "[p]", "[b]", "[t]", "[d]", "[k]", "[g]" },
    fricatives = { "[f]", "[v]", "[s]", "[z]", "[ʃ]", "[ç]", "[h]" },
    nasals = { "[m]", "[n]" },
    liquids = { "[l]", "[ʁ]" }
}

-- Dynamic cluster generation
profile.onsets = {
    { phonemes = { "[p]" }, frequency = 0.8 },
    { phonemes = { "[b]" }, frequency = 0.6 },
    { phonemes = { "[t]" }, frequency = 0.9 },
    { phonemes = { "[d]" }, frequency = 0.7 },
    { phonemes = { "[k]" }, frequency = 0.8 },
    { phonemes = { "[g]" }, frequency = 0.6 },
    { phonemes = { "[f]" }, frequency = 0.7 },
    { phonemes = { "[v]" }, frequency = 0.5 },
    { phonemes = { "[s]" }, frequency = 0.9 },
    { phonemes = { "[z]" }, frequency = 0.4 },
    { phonemes = { "[ʃ]" }, frequency = 0.6 },
    { phonemes = { "[h]" }, frequency = 0.7 },
    { phonemes = { "[m]" }, frequency = 0.8 },
    { phonemes = { "[n]" }, frequency = 0.9 },
    { phonemes = { "[l]" }, frequency = 0.8 },
    { phonemes = { "[ʁ]" }, frequency = 0.7 },
    { phonemes = { "[j]" }, frequency = 0.5 },
    { phonemes = { "[ç]" }, frequency = 0.5 },
    -- Consonant clusters
    { phonemes = { "[ʃ]", "[t]" }, frequency = 0.4 },
    { phonemes = { "[ʃ]", "[p]" }, frequency = 0.3 },
    { phonemes = { "[s]", "[t]" }, frequency = 0.3 },
    { phonemes = { "[s]", "[p]" }, frequency = 0.3 },
    { phonemes = {}, frequency = 0.2 }, -- Empty onset
}

profile.nuclei = {
    { phonemes = { "[a]" }, frequency = 0.9 },
    { phonemes = { "[ɛ]" }, frequency = 0.8 },
    { phonemes = { "[e]" }, frequency = 0.7 },
    { phonemes = { "[ɪ]" }, frequency = 0.8 },
    { phonemes = { "[i]" }, frequency = 0.7 },
    { phonemes = { "[ɔ]" }, frequency = 0.6 },
    { phonemes = { "[o]" }, frequency = 0.5 },
    { phonemes = { "[ʊ]" }, frequency = 0.6 },
    { phonemes = { "[u]" }, frequency = 0.5 },
    { phonemes = { "[aɪ]" }, frequency = 0.5 },
    { phonemes = { "[aʊ]" }, frequency = 0.4 },
    { phonemes = { "[ə]" }, frequency = 0.6 },
}

profile.codas = {
    { phonemes = {}, frequency = 0.3 }, -- Empty coda
    { phonemes = { "[t]" }, frequency = 0.8 },
    { phonemes = { "[n]" }, frequency = 0.9 },
    { phonemes = { "[s]" }, frequency = 0.7 },
    { phonemes = { "[ʁ]" }, frequency = 0.6 },
    { phonemes = { "[l]" }, frequency = 0.6 },
    { phonemes = { "[m]" }, frequency = 0.5 },
    { phonemes = { "[k]" }, frequency = 0.4 },
    { phonemes = { "[p]" }, frequency = 0.3 },
    { phonemes = { "[f]" }, frequency = 0.3 },
    -- Consonant clusters
    { phonemes = { "[n]", "[t]" }, frequency = 0.4 },
    { phonemes = { "[s]", "[t]" }, frequency = 0.4 },
    { phonemes = { "[ʁ]", "[t]" }, frequency = 0.3 },
    { phonemes = { "[l]", "[t]" }, frequency = 0.3 },
}

-- German-specific vowel reduction
function profile.apply_vowel_reduction(syllables, context)
    for i, syllable in ipairs(syllables) do
        if not syllable.stressed and #syllable.nucleus > 0 then
            -- German vowel reduction: unstressed vowels become schwa
            local vowel = syllable.nucleus[1]
            if vowel ~= "[ə]" and vowel ~= "[aɪ]" and vowel ~= "[aʊ]" then
                -- Apply reduction with 80% probability
                if math.random() < 0.8 then
                    syllable.nucleus[1] = "[ə]"
                end
            end
        end
    end
end

-- German consonant simplification
function profile.apply_consonant_simplification(syllables, context)
    for i, syllable in ipairs(syllables) do
        -- Simplify complex consonant clusters
        if #syllable.onset > 2 then
            -- Keep only first two consonants
            local new_onset = { syllable.onset[1], syllable.onset[2] }
            syllable.onset = new_onset
        end

        if #syllable.coda > 2 then
            -- Keep only first two consonants
            local new_coda = { syllable.coda[1], syllable.coda[2] }
            syllable.coda = new_coda
        end
    end
end

-- German final devoicing
function profile.apply_final_devoicing(syllables, context)
    for i, syllable in ipairs(syllables) do
        if #syllable.coda > 0 then
            local final_consonant = syllable.coda[#syllable.coda]

            -- Apply final devoicing rules
            if final_consonant == "[b]" then
                syllable.coda[#syllable.coda] = "[p]"
            elseif final_consonant == "[d]" then
                syllable.coda[#syllable.coda] = "[t]"
            elseif final_consonant == "[g]" then
                syllable.coda[#syllable.coda] = "[k]"
            elseif final_consonant == "[v]" then
                syllable.coda[#syllable.coda] = "[f]"
            elseif final_consonant == "[z]" then
                syllable.coda[#syllable.coda] = "[s]"
            end
        end
    end
end

-- German vowel harmony (front/back preference)
function profile.apply_vowel_harmony(syllables, context)
    if #syllables == 0 then
        return
    end

    local front_vowels = { "[i]", "[ɪ]", "[e]", "[ɛ]" }
    local back_vowels = { "[u]", "[ʊ]", "[o]", "[ɔ]", "[a]" }

    -- Simple vowel harmony implementation
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
                    if current_vowel == back_vowel and front_vowels[j] then
                        syllables[i].nucleus[1] = front_vowels[j]
                        break
                    end
                end
            else
                -- Convert front vowels to back
                for j, front_vowel in ipairs(front_vowels) do
                    if current_vowel == front_vowel and back_vowels[j] then
                        syllables[i].nucleus[1] = back_vowels[j]
                        break
                    end
                end
            end
        end
    end
end

-- Main harmony rule dispatcher
function profile.apply_harmony_rules(syllables, context)
    -- Apply German-specific phonetic rules
    profile.apply_vowel_reduction(syllables, context)
    profile.apply_consonant_simplification(syllables, context)
    profile.apply_final_devoicing(syllables, context)
    profile.apply_vowel_harmony(syllables, context)
end

-- Word composition rules
profile.word_composition = {
    min_syllables = 1,
    max_syllables = 3,
    prefixes = {
        { grapheme = "ge", phonemes = { "[g]", "[ə]" }, frequency = 0.3 },
        { grapheme = "ver", phonemes = { "[f]", "[ɛ]", "[ʁ]" }, frequency = 0.2 },
        { grapheme = "ent", phonemes = { "[ɛ]", "[n]", "[t]" }, frequency = 0.15 },
        { grapheme = "un", phonemes = { "[ʊ]", "[n]" }, frequency = 0.25 },
    },
    suffixes = {
        { grapheme = "er", phonemes = { "[ɛ]", "[ʁ]" }, frequency = 0.4 },
        { grapheme = "in", phonemes = { "[ɪ]", "[n]" }, frequency = 0.3 },
        { grapheme = "chen", phonemes = { "[ç]", "[ə]", "[n]" }, frequency = 0.2 },
        { grapheme = "lein", phonemes = { "[l]", "[aɪ]", "[n]" }, frequency = 0.1 },
    },
    forbidden_transitions = {
        { coda = { "[k]" }, onset = { "[g]" }, forbidden = true },
        { coda = { "[p]" }, onset = { "[b]" }, forbidden = true },
        { coda = { "[t]" }, onset = { "[d]" }, forbidden = true },
        { coda = { "[s]" }, onset = { "[z]" }, forbidden = true },
        { coda = { "[f]" }, onset = { "[v]" }, forbidden = true },
    }
}

-- Style rules
profile.style_rules = {
    harmony_rules = {
        { name = "front_vowel_harmony",         condition = "contains_front_vowel",  requirement = "prefer_front_vowels",     strength = 0.7 },
        { name = "back_vowel_harmony",          condition = "contains_back_vowel",   requirement = "prefer_back_vowels",      strength = 0.6 },
        { name = "consonant_cluster_avoidance", condition = "has_consonant_cluster", requirement = "avoid_adjacent_clusters", strength = 0.8 },
        { name = "vowel_length_consistency",    condition = "has_long_vowel",        requirement = "prefer_long_vowels",      strength = 0.5 },
    },
    frequency_adjustments = {
        word_initial = 1.2,
        word_medial = 1.0,
        word_final = 1.1,
        after_consonant = 0.9,
        after_vowel = 1.1,
        stressed_syllable = 1.3,
        unstressed_syllable = 0.8,
    }
}

-- Export profile
return profile
