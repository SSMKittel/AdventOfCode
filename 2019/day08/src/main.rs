fn main() {
    let img = include_str!("input.txt").chars().collect::<Vec<_>>();

    let checksum = img.chunks(25 * 6)
        .map(|layer|
            (
                layer.iter().filter(|&&pixel| pixel == '0').count(),
                layer.iter().filter(|&&pixel| pixel == '1').count()
                * layer.iter().filter(|&&pixel| pixel == '2').count()
            )
        )
        .min_by_key(|(c0, _)| *c0)
        .map(|(_, cs)| cs);
    println!("Checksum {:?}", checksum.unwrap());

    let mut img_rendered = ['2'; 25 * 6];
    for layer in img.chunks(25 * 6) {
        for i in 0..(25 * 6) {
            if img_rendered[i] == '2' {
                img_rendered[i] = layer[i];
            }
        }
    }
    println!("Image:");
    for r in img_rendered.chunks(25) {
        // Colours inverted because it's easier to read
        println!("{}", r.iter().map(|c| if *c == '1' {'█'} else {' '}).collect::<String>());
    }
}
