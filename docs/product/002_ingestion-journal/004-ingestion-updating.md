# [IJ-004] Ingestion Updating

The application now supports updating existing ingestion records. This feature allows users to modify details of previously logged ingestions, providing flexibility and the ability to correct or enhance recorded information.

## Usage

Users can update an ingestion using the following command:

```
neuronek ingestion update [OPTIONS]
```

### Options

- `--id <ID>`: The ID of the ingestion to update (required)
- `--substance-name <NAME>`: Update the substance name
- `--route-of-administration <ROUTE>`: Update the route of administration
- `--dosage <DOSAGE>`: Update the dosage
- `--notes <NOTES>`: Update or add notes
- `--ingested-at <DATETIME>`: Update the ingestion date and time

## Example

```
> neuronek ingestion update --id 1 --substance-name "Caffeine (corrected)" --dosage 150 --notes "Updated dosage and name for accuracy"

Ingestion updated successfully.
```

## Story

Alex, a regular user of the Neuronek application, realizes they made a mistake when logging their caffeine intake earlier in the day. They initially logged it as "Coffee" with a dosage of 100mg, but after checking their energy drink can, they realize it actually contained 150mg of caffeine.

Alex opens their terminal and uses the Neuronek CLI to update the ingestion:

```
> neuronek ingestion update --id 42 --substance-name "Caffeine (Energy Drink)" --dosage 150 --notes "Corrected dosage after checking the can"

Ingestion updated successfully.
```

Relieved that they could easily correct their mistake, Alex feels more confident in the accuracy of their ingestion journal. This feature allows them to maintain a precise record of their substance intake, which is crucial for their personal health tracking and analysis.

The ability to update ingestions also proves useful when Alex later learns more details about a supplement they took. They can now go back and add this information to the relevant ingestion records, enriching their data over time.
