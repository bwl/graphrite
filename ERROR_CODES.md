# Graphrite Error Codes

Code  Name                             Description
E0001 MissingDirectionFirstLine        Direction must be first non-comment line
E0003 UnquotedMultiwordLabel           Node label must be quoted
E0010 ExpectedBracketAfterLabel        Missing closing ] after label
E0100 InvalidIdentifierSnakeCase       Identifiers must match [a-z][a-z0-9_]* and not end with _
E0201 EdgeFromUnknown                  Edge references unknown source node
E0202 EdgeToUnknown                    Edge references unknown destination node
E0203 OrphanNode                       Node has zero incident edges
E0300 ExceedsMaxLineLength             Line exceeds 100 characters
