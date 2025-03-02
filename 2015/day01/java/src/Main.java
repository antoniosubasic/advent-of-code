import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;

public class Main {
    public static void main(String[] args) throws Exception {
        var input = Files.readString(Paths.get("../input.txt")).chars();

        var index = new AtomicInteger(0);
        var found = new AtomicBoolean(false);

        System.out.println(
            input
                .map(c -> c == '(' ? 1 : -1)
                .reduce(0, (floor, change) -> {
                    var newFloor = floor + change;

                    if (!found.get()) {
                        index.incrementAndGet();
                        if (newFloor < 0) found.set(true);
                    }

                    return newFloor;
                })
        );

        System.out.println(index.get());
    }
}
