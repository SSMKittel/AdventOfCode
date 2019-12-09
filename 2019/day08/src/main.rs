fn main() {
    let img = include_str!("input.txt").chars().collect::<Vec<_>>();
    let mut layer_id = 1;

    let checksum = img.chunks(25 * 6)
        .map(|layer|
            (
                layer.iter().filter(|&&pixel| pixel == '0').count(),
                layer.iter().filter(|&&pixel| pixel == '1').count()
                * layer.iter().filter(|&&pixel| pixel == '2').count()
            )
        )
        .min_by_key(|(c0, cs)| *c0)
        .map(|(c0, cs)| cs);
    println!("{:?}", checksum);
}
