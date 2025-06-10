# varlinker

![](https://github.com/IBCHgenomic/eVaiutilities/blob/main/logo.png)

- Parallel threaded variant linker.
- sample vcf file from [vcflib](https://github.com/vcflib/vcflib/blob/master/samples/sample.vcf)


```
specific position annotator for human genomics.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      Prof. Luiza Handschuh
      Email: luizahan@ibch.poznan.pl.
      ************************************************

Usage: varlinker <COMMAND>

Commands:
  variantlinker   annotate the specific coordinate
  variantrefanno  extract the annotation of the specific ref allele
  variantaltanno  extract the annotation of the specific alt allele
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

- Annotate only the ref variant with A
```
./target/debug/varlinker variant-trefanno ./sample-files/sample.vcf A
```

- Annotate only the alt variant with A
```
./target/debug/varlinker variant-taltanno ./sample-files/sample.vcf T
```
- Annotate all the variants in the vcf
```
./target/debug/varlinker variant-linker ./sample-files/sample.vcf
```

- it will produce three output files classifying variant annotation to gene, exon and transcript level.

```
==> annotationfile-exon.txt <==
19      111     .       A       C       exon    ENSG00000284900.2       KBTBD4
19      111     .       A       C       exon    ENSG00000284900.2       KBTBD4
19      111     .       A       C       exon    ENSG00000284900.2       KBTBD4
19      111     .       A       C       exon    ENSG00000284900.2       KBTBD4

==> annotationfile-gene.txt <==
19      111     .       A       C       gene    ENSG00000284900.2       KBTBD4
19      111     .       A       C       gene    ENSG00000278550.4       SLC43A2
19      111     .       A       C       gene    ENSG00000264450.1       ENSG00000264450
19      111     .       A       C       gene    ENSG00000273929.1       U6

==> annotationfile-transcript.txt <==
19      111     .       A       C       transcript      ENSG00000284900.2       KBTBD4
19      111     .       A       C       transcript      ENSG00000284900.2       KBTBD4
19      111     .       A       C       transcript      ENSG00000284900.2       KBTBD4
19      111     .       A       C       transcript      ENSG00000284900.2       KBTBD4
```

- Acknowledgements: MOSAIC platform, developed as part of the ECBiG-MOSAIC project (POIR.04.02.00-00-D017/20), co-financed by the European Regional Development Fund (ERDF) under the Smart Growth Operational Programme 2014-2020, Measure 4.2 for the development of modern research infrastructure in the science sector.
- Project PI and Informal queries: Prof. Luiza Handschuh: luizahan@ibch.poznan.pl.
- Code related queries: Dr. Gaurav Sablok: gsablok@ibch.poznan.pl.

Gaurav Sablok Instytut Chemii Bioorganicznej Polskiej Akademii Nauk ul. Noskowskiego 12/14 | 61-704, Poznań Poland
